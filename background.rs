use super::{MeshScratch, Mesher, SectionMesh, TerrainSource, packed};
use crate::{chunk::Chunk, light::planar_lighting as lighting};
use horizon_world::planar_plane as plane;

const BACKGROUND_Z: usize = plane::BACKGROUND as usize;

pub(super) fn mesh(
    mesher: &mut Mesher<'_>,
    region: &[[Option<&Chunk>; 3]; 3],
    sy: i32,
    out: SectionMesh,
    scratch: &mut MeshScratch,
    tables: &crate::light::LightTables,
) -> SectionMesh {
    let columns = &region[1];
    let has_background = columns.map(|chunk| chunk.is_some_and(|c| has_wall(c, sy)));
    let foreground = std::array::from_fn::<_, 3, _>(|i| {
        columns[i]
            .filter(|_| has_background[i])
            .map(|c| isolate(c, sy, false))
    });
    let mut front_region = *region;
    front_region[1] = std::array::from_fn(|i| foreground[i].as_ref().or(columns[i]));
    let mut front = mesher.mesh_reusing(&front_region, sy, out, scratch);
    super::planar_canopy::cover_trunks(
        &mut front,
        columns[1].expect("center chunk"),
        sy,
        mesher.assets.registry,
    );
    lighting::shade(&mut front, columns, sy, tables);

    if has_background[1] {
        let walls = columns.map(|c| c.map(|c| isolate(c, sy, true)));
        let walls = [&walls[0], &walls[1], &walls[2]].map(Option::as_ref);
        let mut back = mesher.mesh_reusing(
            &[[None; 3], walls, [None; 3]],
            sy,
            SectionMesh::default(),
            scratch,
        );
        lighting::shade(&mut back, columns, sy, tables);
        merge(&mut front, back);
    }
    front
}

fn sections(chunk: &Chunk, sy: i32) -> std::ops::Range<i32> {
    (sy - 1).max(0)..(sy + 2).min(chunk.sections.len() as i32)
}

fn has_wall(chunk: &Chunk, sy: i32) -> bool {
    sections(chunk, sy).any(|s| {
        let section = &chunk.sections[s as usize];
        (0..16).any(|y| (0..16).any(|x| section.get(y * 256 + BACKGROUND_Z * 16 + x) != 0))
    })
}

fn isolate(chunk: &Chunk, sy: i32, wall: bool) -> Chunk {
    let mut isolated = chunk.clone();
    for s in sections(chunk, sy) {
        let mut blocks = [0; 4096];
        chunk.sections[s as usize].unpack(&mut blocks);
        for row in blocks.chunks_exact_mut(256) {
            let start = BACKGROUND_Z * 16;
            if wall {
                row[..start].fill(0);
                row[start + 16..].fill(0);
            } else {
                row[start..start + 16].fill(0);
            }
        }
        isolated.replace_blocks(s as usize, &blocks);
    }
    isolated
}

fn merge(front: &mut SectionMesh, mut back: SectionMesh) {
    for vertex in back.solid.iter_mut().chain(&mut back.translucent) {
        // Storage is at positive Z. The side camera sees that side of the
        // plane, so move the wall behind the foreground for rendering.
        vertex.pos[2] -= plane::BACKGROUND_DEPTH_SHIFT;
        let mut rgba = vertex.color.to_le_bytes();
        for channel in &mut rgba[..3] {
            *channel = (u16::from(*channel) * 3 / 4) as u8;
        }
        vertex.color = u32::from_le_bytes(rgba);
    }
    let front_solid = front.solid.len() / 4;
    let back_solid = back.solid.len() / 4;
    append_sources(
        &mut front.solid_sources,
        back.solid_sources,
        front_solid,
        back_solid,
    );
    append_sources(
        &mut front.translucent_sources,
        back.translucent_sources,
        front.translucent.len() / 4,
        back.translucent.len() / 4,
    );
    if back_solid > 0 {
        front
            .solid_tints
            .resize(front_solid + back_solid, packed::UNKNOWN_TINT);
    }
    front.solid.append(&mut back.solid);
    front.translucent.append(&mut back.translucent);
    front.sprite_layers.append(&mut back.sprite_layers);
    front.finish_sprite_layers();
}

fn append_sources(
    front: &mut Vec<TerrainSource>,
    mut back: Vec<TerrainSource>,
    front_quads: usize,
    back_quads: usize,
) {
    if !front.is_empty() || !back.is_empty() {
        front.resize(front_quads, TerrainSource::default());
        front.append(&mut back);
        front.resize(front_quads + back_quads, TerrainSource::default());
    }
}

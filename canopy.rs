use super::{SectionMesh, TerrainSource, TerrainVertex, packed};
use crate::chunk::Chunk;
use horizon_world::registry::BlockRegistry;

const FRONT_Z: f32 = 1.0;
const LEAF_DEPTH_OFFSET: f32 = 1.0 / 16.0;

pub(super) fn cover_trunks(
    mesh: &mut SectionMesh,
    chunk: &Chunk,
    sy: i32,
    registry: &BlockRegistry,
) {
    let Some(log) = registry.default_state("minecraft:oak_log") else {
        return;
    };
    let Some(mut leaf) = registry
        .default_state("minecraft:oak_leaves")
        .and_then(|id| registry.state(id))
        .cloned()
    else {
        return;
    };
    leaf.properties.insert("persistent".into(), "false".into());
    leaf.properties.insert("distance".into(), "1".into());
    let Ok(leaf) = registry.resolve(&leaf) else {
        return;
    };
    let bottom = chunk.min_y() + sy * 16;
    for (vertices, sources, mut tints) in [
        (
            &mut mesh.solid,
            &mut mesh.solid_sources,
            Some(&mut mesh.solid_tints),
        ),
        (&mut mesh.translucent, &mut mesh.translucent_sources, None),
    ] {
        for index in 0..vertices.len() / 4 {
            let quad: [TerrainVertex; 4] = vertices[index * 4..index * 4 + 4]
                .try_into()
                .expect("complete terrain quad");
            if !quad.iter().all(|v| (v.pos[2] - FRONT_Z).abs() < 1e-4) {
                continue;
            }
            let [x, local_y] = [0, 1]
                .map(|axis| (quad.iter().map(|v| v.pos[axis]).sum::<f32>() * 0.25).floor() as i32);
            let y = bottom + local_y;
            if !(0..14).contains(&x)
                || !(0..16).contains(&local_y)
                || chunk.get(x, y, 0) != leaf.0
                || chunk.get(x + 1, y, 0) != log.0
                || chunk.get(x + 2, y, 0) != leaf.0
            {
                continue;
            }
            // The left and right inner leaves identify the two rows where
            // the generated crown overlaps its trunk. Reuse the left leaf's
            // art and tint to draw a face just in front of the log.
            vertices.extend(quad.map(|mut v| {
                v.pos[0] += 1.0;
                v.pos[2] += LEAF_DEPTH_OFFSET;
                v
            }));
            if !sources.is_empty() {
                sources.resize(
                    vertices.len() / 4,
                    TerrainSource::new(leaf.0, [x + 1, local_y, 0], 0, false),
                );
            }
            if let Some(tints) = tints.as_deref_mut() {
                let tint = tints.get(index).copied().unwrap_or(packed::UNKNOWN_TINT);
                tints.resize(vertices.len() / 4 - 1, packed::UNKNOWN_TINT);
                tints.push(tint);
            }
        }
    }
}

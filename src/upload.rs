use super::*;
use horizon_client::mesher::{MeshScratch, Mesher, SectionMesh};

pub(super) fn prepare(game: &Game<'_>, uploads: &mut [((i32, i32, i32), SectionMesh)]) {
    if !super::planar_side::local(game) {
        return;
    }
    let context = game.worker_context();
    let streaming = &game.streaming;
    let mut mesher = Mesher {
        assets: &context.assets,
        render: &context.render,
        tables: &context.tables,
        biomes: &context.biomes,
        visibility: context.tables.visibility.clone(),
        nether_shading: streaming.nether_shading,
        shader_pack_lighting: streaming.shader_pack_lighting,
        separate_ao: streaming.separate_ao,
        shader_pack: streaming.shader_pack,
        looks: streaming.looks,
        zoom: streaming.biome_zoom,
        blend_radius: streaming.biome_blend,
        smooth_lighting: streaming.smooth_lighting,
        cutout_leaves: streaming.cutout_leaves,
        opaque_ice: streaming.opaque_ice,
        translucent_layers: &streaming.translucent_layers,
        leaf_culling: streaming.leaf_culling,
        hidden_states: &context.hidden_states,
        part_tables: &context.part_tables,
    };
    let mut scratch = MeshScratch::default();
    for ((cx, sy, cz), mesh) in uploads {
        if *cz != 0 {
            continue;
        }
        let mut region = [[None; 3]; 3];
        for (x, cell) in region[1].iter_mut().enumerate() {
            *cell = game.world.chunks.get(&(*cx + x as i32 - 1, 0)).map(Arc::as_ref);
        }
        if region[1][1].is_none() {
            continue;
        }
        *mesh = super::planar_background::mesh(
            &mut mesher, &region, *sy, SectionMesh::default(), &mut scratch, &context.light,
        );
        mesh.sort_solid();
    }
}

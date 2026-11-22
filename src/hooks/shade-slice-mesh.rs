fn shade_slice_mesh(
    mesher: &mut Mesher<'_>,
    region: &[[Option<&Chunk>; 3]; 3],
    sy: i32,
    out: &mut SectionMesh,
    scratch: &mut MeshScratch,
) -> ::core::ops::ControlFlow<SectionMesh> {
    use crate::game::planar_lighting as lighting;
    let Some(tables) = region[1][1].and_then(lighting::tables) else {
        return std::ops::ControlFlow::Continue(());
    };
    std::ops::ControlFlow::Break(lighting::unshaded(|| {
        planar_background::mesh(mesher, region, sy, std::mem::take(out), scratch, &tables)
    }))
}

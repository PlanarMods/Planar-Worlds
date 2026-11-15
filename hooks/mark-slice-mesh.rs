fn mark_slice_mesh(world: &World, x: i32, z: i32) {
    if z == 0
        && crate::light::planar_lighting::capturing()
        && let Some(chunk) = world.chunks.get(&(x, z))
    {
        crate::light::planar_lighting::tag(chunk);
    }
}

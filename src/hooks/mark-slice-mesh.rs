fn mark_slice_mesh(world: &World, x: i32, z: i32) {
    if z == 0
        && crate::game::planar_lighting::capturing()
        && let Some(chunk) = world.chunks.get(&(x, z))
    {
        crate::game::planar_lighting::tag(chunk);
    }
}

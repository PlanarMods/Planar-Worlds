fn background_collision(position: BlockPos) -> ::core::ops::ControlFlow<io::Result<VoxelShape>> {
    use horizon_world::planar_plane as plane;
    if plane::active() && position.z == plane::BACKGROUND {
        std::ops::ControlFlow::Break(Ok(VoxelShape::empty()))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

fn side_spread(to: BlockPos, canceled: &mut bool) -> ::core::ops::ControlFlow<()> {
    use horizon_world::planar_plane as plane;
    if plane::active() && to.z != plane::layer() {
        *canceled = true;
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

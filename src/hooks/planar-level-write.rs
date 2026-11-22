fn planar_level_write(pos: BlockPos) -> ::core::ops::ControlFlow<Option<BlockStateId>> {
    use mod_planar_worlds::plane;
    if plane::active() && pos.z != plane::layer() {
        std::ops::ControlFlow::Break(None)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

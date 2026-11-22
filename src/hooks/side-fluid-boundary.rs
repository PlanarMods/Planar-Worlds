fn side_fluid_boundary(
    level: &GameLevel<'_>,
    pos: BlockPos,
) -> ::core::ops::ControlFlow<BlockStateId> {
    use mod_planar_worlds::plane as plane;
    if plane::active()
        && pos.z != plane::layer()
        && let Some(wall) = level
            .collision
            .registry()
            .default_state("minecraft:barrier")
    {
        return std::ops::ControlFlow::Break(wall);
    }
    std::ops::ControlFlow::Continue(())
}

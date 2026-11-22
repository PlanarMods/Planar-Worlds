fn side_falling_bounds(entity: &FallingBlock) -> ::core::ops::ControlFlow<Aabb> {
    use mod_planar_worlds::plane as plane;
    if plane::active() {
        std::ops::ControlFlow::Break(crate::game::planar_side::flattened(|| Entity::bounds(entity)))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

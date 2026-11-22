fn side_trident_bounds(entity: &ThrownTrident) -> ::core::ops::ControlFlow<Aabb> {
    use mod_planar_worlds::plane as plane;
    if plane::active() {
        std::ops::ControlFlow::Break(crate::game::planar_side::flattened(|| Entity::bounds(entity)))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

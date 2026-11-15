fn side_trident_bounds(entity: &ThrownTrident) -> ::core::ops::ControlFlow<Aabb> {
    use horizon_world::planar_plane as plane;
    if plane::active() {
        std::ops::ControlFlow::Break(plane::flatten(|| Entity::bounds(entity)))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

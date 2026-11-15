fn side_projectile_bounds(entity: &Projectile) -> ::core::ops::ControlFlow<Aabb> {
    if horizon_world::planar_plane::active() {
        std::ops::ControlFlow::Break(horizon_world::planar_plane::flatten(|| entity.bounds()))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

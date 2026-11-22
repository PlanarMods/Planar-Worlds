fn side_boat_bounds(entity: &Boat) -> ::core::ops::ControlFlow<Aabb> {
    if mod_planar_worlds::plane::active() {
        std::ops::ControlFlow::Break(crate::game::planar_side::flattened(|| entity.bounds()))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

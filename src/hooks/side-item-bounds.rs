fn side_item_bounds(entity: &ItemEntity) -> ::core::ops::ControlFlow<Aabb> {
    if mod_planar_worlds::plane::active() {
        std::ops::ControlFlow::Break(crate::game::planar_side::flattened(|| entity.bounds()))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

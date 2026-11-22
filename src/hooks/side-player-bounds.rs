fn side_player_bounds(player: &crate::motion::PlayerMotion) -> ::core::ops::ControlFlow<Aabb> {
    if mod_planar_worlds::plane::active() {
        std::ops::ControlFlow::Break(crate::game::planar_side::flatten_bounds(
            player.dimensions.bounds(player.position),
        ))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

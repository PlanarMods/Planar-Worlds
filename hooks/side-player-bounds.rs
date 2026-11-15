fn side_player_bounds(player: &crate::motion::PlayerMotion) -> ::core::ops::ControlFlow<Aabb> {
    if horizon_world::planar_plane::active() {
        std::ops::ControlFlow::Break(horizon_world::planar_plane::bounds(
            player.dimensions.bounds(player.position),
        ))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

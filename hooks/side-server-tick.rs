fn side_server_tick(
    game: &mut crate::game::Game<'_>,
    running: bool,
) -> ::core::ops::ControlFlow<Result<()>> {
    use horizon_world::planar_plane as plane;
    if plane::active() || !crate::game::planar_side::server(game) {
        return std::ops::ControlFlow::Continue(());
    }
    crate::game::planar_side::center_entities(game);
    let result = plane::scoped(|| game.tick_world(running));
    crate::game::planar_side::center_entities(game);
    std::ops::ControlFlow::Break(result)
}

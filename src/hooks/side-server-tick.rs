fn side_server_tick(
    game: &mut crate::game::Game<'_>,
    running: bool,
) -> ::core::ops::ControlFlow<Result<()>> {
    use mod_planar_worlds::plane as plane;
    if plane::active() || !crate::game::planar_side::server(game) {
        return std::ops::ControlFlow::Continue(());
    }
    crate::game::planar_side::center_entities(game);
    crate::game::planar_side::ensure_land_spawn(game);
    crate::game::planar_side::clamp_server_players(game);
    let result = plane::scoped(|| game.tick_world(running));
    crate::game::planar_side::center_entities(game);
    std::ops::ControlFlow::Break(result)
}

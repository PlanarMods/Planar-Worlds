fn planar_player_physics(
    game: &mut crate::game::Game<'_>,
    entity_collisions: &[horizon_core::geometry::Aabb],
    pushed: Vec3,
    sprint_window: i32,
) -> ::core::ops::ControlFlow<Result<()>> {
    if !crate::game::planar_side::local(game) || crate::game::planar_physics::hidden() {
        return std::ops::ControlFlow::Continue(());
    }
    crate::game::planar_side::center(game);
    let pushed = Vec3::new(pushed.x, pushed.y, 0.0);
    let collisions = entity_collisions.to_vec();
    let result = crate::game::planar_physics::run(game, |game| {
        game.tick_local_player(collisions, pushed, sprint_window)
    });
    crate::game::planar_side::center(game);
    std::ops::ControlFlow::Break(result)
}

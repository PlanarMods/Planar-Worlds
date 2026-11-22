fn planar_mob_physics(game: &mut crate::game::Game<'_>) -> ::core::ops::ControlFlow<Result<()>> {
    if !mod_planar_worlds::plane::active() || crate::game::planar_physics::hidden() {
        return std::ops::ControlFlow::Continue(());
    }
    crate::game::planar_side::center_entities(game);
    let result = crate::game::planar_physics::run(game, |game| game.tick_creatures());
    crate::game::planar_side::center_entities(game);
    std::ops::ControlFlow::Break(result)
}

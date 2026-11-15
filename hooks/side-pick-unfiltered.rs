fn side_pick_unfiltered(game: &crate::game::Game<'_>) -> ::core::ops::ControlFlow<Option<Hit>> {
    if !crate::game::planar_side::local(game) {
        return std::ops::ControlFlow::Continue(());
    }
    let reach = game.block_reach.max(game.entity_reach);
    std::ops::ControlFlow::Break(
        game.forced_hit
            .or_else(|| crate::game::planar_side::pick(game, reach)),
    )
}

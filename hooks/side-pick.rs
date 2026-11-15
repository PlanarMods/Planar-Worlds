fn side_pick(game: &crate::game::Game<'_>) -> ::core::ops::ControlFlow<Option<Hit>> {
    if !crate::game::planar_side::local(game) {
        return std::ops::ControlFlow::Continue(());
    }
    if let Some(hit) = game.forced_hit {
        return std::ops::ControlFlow::Break(Some(hit));
    }
    if let Some(hit) = game.tick_hit {
        return std::ops::ControlFlow::Break(hit);
    }
    std::ops::ControlFlow::Break(crate::game::planar_side::pick(game, game.block_reach))
}

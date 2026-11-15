fn side_pick_fluid(game: &crate::game::Game<'_>) -> ::core::ops::ControlFlow<Option<Hit>> {
    if crate::game::planar_side::local(game) {
        std::ops::ControlFlow::Break(crate::game::planar_side::pick_fluid(game))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

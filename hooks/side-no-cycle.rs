fn side_no_cycle(game: &crate::game::Game<'_>) -> ::core::ops::ControlFlow<()> {
    if crate::game::planar_side::local(game) {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

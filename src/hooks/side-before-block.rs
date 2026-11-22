fn side_before_block(
    game: &crate::game::Game<'_>,
    distance: f64,
) -> ::core::ops::ControlFlow<bool> {
    if crate::game::planar_side::local(game) {
        std::ops::ControlFlow::Break(crate::game::planar_side::entity_before_cursor(
            game, distance,
        ))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

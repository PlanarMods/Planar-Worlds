fn side_aim(
    game: &crate::game::Game<'_>,
) -> ::core::ops::ControlFlow<Option<([f64; 3], [f64; 3])>> {
    if crate::game::planar_side::local(game) {
        std::ops::ControlFlow::Break(crate::game::planar_side::aim(game))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

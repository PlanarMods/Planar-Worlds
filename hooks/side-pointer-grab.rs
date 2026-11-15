fn side_pointer_grab(game: &crate::game::Game<'_>) -> ::core::ops::ControlFlow<bool> {
    if crate::game::planar_side::free_pointer(game) {
        std::ops::ControlFlow::Break(true)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

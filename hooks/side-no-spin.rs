fn side_no_spin(game: &crate::game::Game<'_>) -> ::core::ops::ControlFlow<glam::Mat4> {
    if crate::game::planar_side::local(game) {
        std::ops::ControlFlow::Break(glam::Mat4::IDENTITY)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

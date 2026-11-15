fn side_camera_hook(
    game: &crate::game::Game<'_>,
    view: (glam::DVec3, f32, f32),
) -> ::core::ops::ControlFlow<(glam::DVec3, f32, f32)> {
    if crate::game::planar_side::local(game) {
        std::ops::ControlFlow::Break(view)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

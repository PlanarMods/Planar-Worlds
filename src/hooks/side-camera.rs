fn side_camera(
    game: &crate::game::Game<'_>,
    partial: f32,
) -> ::core::ops::ControlFlow<Option<(glam::DVec3, f32, f32)>> {
    if crate::game::planar_side::local(game) {
        std::ops::ControlFlow::Break(crate::game::planar_side::camera(game, partial))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

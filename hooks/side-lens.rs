fn side_lens(game: &crate::game::Game<'_>) -> ::core::ops::ControlFlow<f32> {
    if crate::game::planar_side::local(game) {
        std::ops::ControlFlow::Break(-horizon_world::planar_plane::HALF_HEIGHT)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

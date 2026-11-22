fn side_no_bob(game: &crate::game::Game<'_>) -> ::core::ops::ControlFlow<glam::Mat4> {
    if crate::game::planar_side::local(game) {
        let size = game.renderer.as_ref().map_or([1280, 720], |r| [r.config.width, r.config.height]);
        let aspect = size[0] as f32 / size[1].max(1) as f32;
        let far = horizon_client::camera::depth_far(game.terrain_distance(), game.settings.cloud_range);
        let half_y = mod_planar_worlds::plane::HALF_HEIGHT;
        let perspective = glam::Mat4::perspective_rh(70.0_f32.to_radians(), aspect, far, 0.05);
        let orthographic = glam::Mat4::orthographic_rh(-half_y * aspect, half_y * aspect, -half_y, half_y, far, 0.05);
        std::ops::ControlFlow::Break(perspective.inverse() * orthographic)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

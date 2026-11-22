fn side_client_tick(game: &mut crate::game::Game<'_>) -> ::core::ops::ControlFlow<Result<()>> {
    use mod_planar_worlds::plane as plane;
    if plane::active() || !crate::game::planar_side::local(game) {
        return std::ops::ControlFlow::Continue(());
    }
    crate::game::planar_side::center(game);
    crate::game::planar_side::movement_pose(game);
    let saved = game.camera_type;
    game.camera_type = crate::camera::CameraType::ThirdPersonBack;
    let result = plane::scoped(|| game.tick_remote());
    game.camera_type = saved;
    crate::game::planar_side::center(game);
    crate::game::planar_side::center_entities(game);
    std::ops::ControlFlow::Break(result)
}

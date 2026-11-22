fn side_render(game: &mut crate::game::Game<'_>) -> ::core::ops::ControlFlow<Result<bool>> {
    use mod_planar_worlds::plane as plane;
    crate::game::planar_side::sync_pointer(game);
    if plane::active() || !crate::game::planar_side::local(game) {
        return std::ops::ControlFlow::Continue(());
    }
    let saved = game.camera_type;
    game.camera_type = crate::camera::CameraType::ThirdPersonBack;
    let player_pose = crate::game::planar_side::snap_model_facing(game);
    let mut mob_poses = crate::game::planar_side::mob_poses(game);
    let result = plane::scoped(|| game.render());
    if let (Some(pose), Some(player)) = (player_pose, game.player_model.as_mut()) {
        pose.restore(player);
    }
    for mob in game.mobs.iter_mut() {
        if let Some(pose) = mob_poses.remove(&mob.id) {
            pose.restore(mob);
        }
    }
    game.camera_type = saved;
    std::ops::ControlFlow::Break(result)
}

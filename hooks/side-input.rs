fn side_input(game: &crate::game::Game<'_>, forward: &mut f32, strafe: &mut f32) {
    if crate::game::planar_side::local(game) {
        *forward = if game.yaw > 0.0 { *strafe } else { -*strafe };
        *strafe = 0.0;
    }
}

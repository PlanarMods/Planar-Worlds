fn side_player_move(
    motion: &mut crate::motion::MotionWorld<'_, '_, '_>,
    movement: Vec3,
) -> ::core::ops::ControlFlow<Result<()>> {
    if !horizon_world::planar_plane::active() {
        return std::ops::ControlFlow::Continue(());
    }
    motion.player.velocity.z = 0.0;
    if movement.z == 0.0 {
        std::ops::ControlFlow::Continue(())
    } else {
        std::ops::ControlFlow::Break(motion.move_self(Vec3::new(movement.x, movement.y, 0.0)))
    }
}

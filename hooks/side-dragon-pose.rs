fn side_dragon_pose(
    dragon: &Dragon,
    delay: usize,
    partial: f32,
) -> ::core::ops::ControlFlow<(f64, f32)> {
    if !horizon_world::planar_plane::active() {
        return std::ops::ControlFlow::Continue(());
    }
    let partial = if dragon.alive() {
        f64::from(partial)
    } else {
        0.0
    };
    let a = dragon.sample(delay);
    let b = dragon.sample(delay + 1);
    let yaw = crate::game::planar_pointer::mob_yaw(
        dragon.position.x - dragon.previous.x,
        dragon.yaw,
        -90.0,
    );
    std::ops::ControlFlow::Break((b.0 + (a.0 - b.0) * partial, yaw))
}

fn side_mob_bounds(mob: &crate::mobs::Mob, position: Vec3) -> ::core::ops::ControlFlow<Aabb> {
    if !mod_planar_worlds::plane::active() {
        return std::ops::ControlFlow::Continue(());
    }
    let (width, height, _) = mob.dims();
    let radius = f64::from(width as f32) / 2.0;
    std::ops::ControlFlow::Break(crate::game::planar_side::flatten_bounds(Aabb::new(
        Vec3::new(position.x - radius, position.y, position.z),
        Vec3::new(
            position.x + radius,
            position.y + f64::from(height as f32),
            position.z,
        ),
    )))
}

fn side_ray(
    game: &crate::game::Game<'_>,
    origin: [f64; 3],
    dir: [f64; 3],
    reach: f64,
) -> ::core::ops::ControlFlow<Option<Hit>> {
    if !horizon_world::planar_plane::active()
        || (origin[2] == horizon_world::planar_plane::Z && dir[2] == 0.0)
    {
        return std::ops::ControlFlow::Continue(());
    }
    let length = dir[0].hypot(dir[1]);
    if !length.is_finite() || length <= f64::EPSILON {
        return std::ops::ControlFlow::Break(None);
    }
    std::ops::ControlFlow::Break(game.clip_ray(
        [origin[0], origin[1], horizon_world::planar_plane::Z],
        [dir[0] / length, dir[1] / length, 0.0],
        reach,
    ))
}

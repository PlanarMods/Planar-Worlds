fn side_track(game: &mut crate::game::Game<'_>) {
    if horizon_world::planar_plane::active() {
        crate::game::planar_side::center_entities(game);
    }
}

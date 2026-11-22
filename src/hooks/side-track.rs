fn side_track(game: &mut crate::game::Game<'_>) {
    if mod_planar_worlds::plane::active() {
        crate::game::planar_side::center_entities(game);
    }
}

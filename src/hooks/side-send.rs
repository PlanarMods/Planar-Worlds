fn side_send(game: &mut crate::game::Game<'_>) {
    if crate::game::planar_side::local(game) {
        crate::game::planar_side::center(game);
    }
}

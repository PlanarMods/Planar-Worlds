fn side_default_outline(game: &crate::game::Game<'_>, canceled: &mut bool) {
    if crate::game::planar_side::local(game) {
        *canceled = true;
    }
}

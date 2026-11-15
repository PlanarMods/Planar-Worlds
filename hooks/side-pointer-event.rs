fn side_pointer_event(game: &mut crate::game::Game<'_>, event: &WindowEvent) {
    crate::game::planar_side::pointer_event(game, event);
}

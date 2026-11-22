fn background_predict_break(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    state: BlockStateId,
    sequence: i32,
) -> ::core::ops::ControlFlow<()> {
    use mod_planar_worlds::plane as plane;
    if !crate::game::planar_side::local(game) || plane::layer() == plane::BACKGROUND {
        return std::ops::ControlFlow::Continue(());
    }
    if pos[2] == 0 && !plane::in_combo_break() {
        // Mirror the cell-clear the server performs: the wall disappears with
        // the foreground, without waiting for the server's block update.
        plane::combo_break(|| {
            plane::background(|| {
                let behind = [pos[0], pos[1], plane::BACKGROUND];
                let wall = game.world.block(behind[0], behind[1], behind[2]);
                if wall.0 != 0 {
                    game.predict_break(behind, wall, sequence);
                    game.world.dirty_sections.insert((
                        behind[0] >> 4,
                        (behind[1] - game.world.shape.min_y) >> 4,
                        0,
                    ));
                }
            });
        });
        return std::ops::ControlFlow::Continue(());
    }
    if pos[2] != plane::BACKGROUND {
        return std::ops::ControlFlow::Continue(());
    }
    plane::background(|| {
        game.predict_break(pos, state, sequence);
        // The predicted removal may be acknowledged without another block write.
        game.world
            .dirty_sections
            .insert((pos[0] >> 4, (pos[1] - game.world.shape.min_y) >> 4, 0));
    });
    std::ops::ControlFlow::Break(())
}

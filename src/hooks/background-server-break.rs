fn background_server_break(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
) -> ::core::ops::ControlFlow<Result<bool>> {
    use mod_planar_worlds::plane as plane;
    if !plane::active() || plane::in_combo_break() {
        return std::ops::ControlFlow::Continue(());
    }
    if pos[2] == plane::BACKGROUND {
        if plane::layer() != plane::BACKGROUND {
            // Ctrl-mining the wall directly.
            return std::ops::ControlFlow::Break(plane::background(|| {
                game.break_block_as_player(pos)
            }));
        }
        return std::ops::ControlFlow::Continue(());
    }
    if pos[2] != 0 || plane::layer() == plane::BACKGROUND || plane::generating() {
        return std::ops::ControlFlow::Continue(());
    }
    // One dig clears the whole cell: break the foreground with the game's own
    // path (loot, parts, events), then remove the wall behind it without a
    // second loot roll, tool damage or statistic.
    let broke = match plane::combo_break(|| game.break_block_as_player(pos)) {
        Ok(broke) => broke,
        Err(error) => return std::ops::ControlFlow::Break(Err(error)),
    };
    if broke {
        let behind = [pos[0], pos[1], plane::BACKGROUND];
        let state = game.world.block(behind[0], behind[1], behind[2]);
        if state.0 != 0 {
            let facts = game.res.facts().get(state).clone();
            let held = game.held().cloned();
            plane::background(|| {
                game.player_destroy_block_removal(behind, state, &facts, held.as_ref(), false);
            });
            game.world.dirty_sections.insert((
                behind[0] >> 4,
                (behind[1] - game.world.shape.min_y) >> 4,
                0,
            ));
        }
    }
    std::ops::ControlFlow::Break(Ok(broke))
}

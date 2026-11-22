fn relight_slice_edit(game: &mut crate::game::Game<'_>, pos: [i32; 3], state: i32) {
    use mod_planar_worlds::plane as plane;
    if !matches!(pos[2], 0 | plane::BACKGROUND)
        || !crate::game::planar_side::local(game)
        || !game.world.loaded(pos[0], 0)
    {
        return;
    }
    let raw = state.max(0) as u32;
    let new = if horizon_client::chunk::known_block_state(raw) {
        raw
    } else {
        0
    };
    let old = game.world.block(pos[0], pos[1], pos[2]).0;
    if old == new || (pos[2] == 0 && !game.pack.context.light.differs(old, new)) {
        return;
    }
    let column = pos[0] >> 4;
    for x in column - 1..=column + 1 {
        if let Some(chunk) = game.world.chunks.get(&(x, 0)) {
            for (sy, section) in chunk.sections.iter().enumerate() {
                if !section.is_uniform(0) {
                    game.world.dirty_sections.insert((x, sy as i32, 0));
                }
            }
        }
    }
}

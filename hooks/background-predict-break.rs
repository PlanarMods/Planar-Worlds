fn background_predict_break(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    state: BlockStateId,
    sequence: i32,
) -> ::core::ops::ControlFlow<()> {
    use horizon_world::planar_plane as plane;
    if !crate::game::planar_side::local(game)
        || pos[2] != plane::BACKGROUND
        || plane::layer() == plane::BACKGROUND
    {
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

fn background_remote_sync(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    state: i32,
    predicted: bool,
) -> ::core::ops::ControlFlow<()> {
    use mod_planar_worlds::plane as plane;
    if !crate::game::planar_side::local(game)
        || pos[2] != plane::BACKGROUND
        || plane::layer() == plane::BACKGROUND
    {
        return std::ops::ControlFlow::Continue(());
    }
    // Re-enters this function with the background layer selected: the guard above then lets the
    // game's own body run, which is what actually stores the block.
    plane::background(|| game.set_block(pos, state, predicted));
    std::ops::ControlFlow::Break(())
}

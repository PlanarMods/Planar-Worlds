fn background_remote_sync(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    state: i32,
) -> ::core::ops::ControlFlow<()> {
    use horizon_world::planar_plane as plane;
    if !crate::game::planar_side::local(game)
        || pos[2] != plane::BACKGROUND
        || plane::layer() == plane::BACKGROUND
    {
        return std::ops::ControlFlow::Continue(());
    }
    plane::background(|| game.set_block(pos, state));
    std::ops::ControlFlow::Break(())
}

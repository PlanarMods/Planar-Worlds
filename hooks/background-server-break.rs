fn background_server_break(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
) -> ::core::ops::ControlFlow<Result<bool>> {
    use horizon_world::planar_plane as plane;
    if plane::active() && pos[2] == plane::BACKGROUND && plane::layer() != plane::BACKGROUND {
        std::ops::ControlFlow::Break(plane::background(|| game.break_block_as_player(pos)))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

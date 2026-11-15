fn background_server_use(
    game: &mut crate::game::Game<'_>,
) -> ::core::ops::ControlFlow<Result<Option<Hand>>> {
    use horizon_world::planar_plane as plane;
    if !plane::active()
        || plane::layer() == plane::BACKGROUND
        || !game
            .forced_hit
            .is_some_and(|hit| hit.pos[2] == plane::BACKGROUND)
    {
        return std::ops::ControlFlow::Continue(());
    }
    std::ops::ControlFlow::Break(plane::background(|| game.use_item_as_player()))
}

fn side_server_move(
    hub: &mut Hub,
    i: usize,
    pos: Option<[f64; 3]>,
    rot: Option<(f32, f32)>,
    flags: u8,
    ctx: &TickCtx<'_>,
    events: &mut Vec<HostEvent>,
) -> ::core::ops::ControlFlow<()> {
    if mod_planar_worlds::plane::active()
        && let Some(mut position) = pos
        && position[2].is_finite()
        && position[2] != mod_planar_worlds::plane::Z
    {
        position[2] = mod_planar_worlds::plane::Z;
        hub.move_player(i, Some(position), rot, flags, ctx, events);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}

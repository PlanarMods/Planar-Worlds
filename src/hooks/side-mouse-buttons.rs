fn side_mouse_buttons(
    game: &mut crate::game::Game<'_>,
    button: MouseButton,
    pressed: bool,
) -> ::core::ops::ControlFlow<Result<()>> {
    if !crate::game::planar_side::local(game) {
        return std::ops::ControlFlow::Continue(());
    }
    let held = match button {
        MouseButton::Left => game.left,
        MouseButton::Right => game.right,
        _ => return std::ops::ControlFlow::Continue(()),
    };
    let clicked = pressed && !held;
    game.mouse_mapping_set_down(button, pressed);
    if clicked {
        game.mouse_mapping_click(button);
    }
    std::ops::ControlFlow::Break(Ok(()))
}

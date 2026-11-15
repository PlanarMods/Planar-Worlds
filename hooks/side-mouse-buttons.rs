fn side_mouse_buttons(
    game: &mut crate::game::Game<'_>,
    button: MouseButton,
    pressed: bool,
) -> ::core::ops::ControlFlow<Result<()>> {
    use horizon_client::interaction_input::Button;
    if !crate::game::planar_side::local(game) {
        return std::ops::ControlFlow::Continue(());
    }
    let (held, slot, action) = match button {
        MouseButton::Left => (&mut game.left, 2, Button::Attack),
        MouseButton::Right => (&mut game.right, 3, Button::Use),
        _ => return std::ops::ControlFlow::Continue(()),
    };
    let clicked = pressed && !*held;
    *held = pressed;
    game.toggles.button(slot, pressed, false, false);
    if clicked && let Some(remote) = game.remote.as_mut() {
        remote.input.press(action);
    }
    std::ops::ControlFlow::Break(Ok(()))
}

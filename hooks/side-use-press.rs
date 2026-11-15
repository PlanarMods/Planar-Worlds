fn side_use_press(game: &mut crate::game::Game<'_>) {
    use crate::game::planar_side as side;
    // Holding use repeats only while the cursor has an eligible empty foreground cell.
    if side::local(game) && game.right && !side::can_repeat_air_placement(game) {
        game.use_delay = game.use_delay.max(1);
    }
}

fn side_reticle(game: &crate::game::Game<'_>, batch: &mut crate::hud::HudBatch, gui_hidden: bool) {
    if !gui_hidden && crate::game::planar_side::local(game) {
        crate::game::planar_side::highlight(game, batch);
    }
}

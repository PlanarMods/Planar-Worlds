use horizon_client::mod_api::{
    ClientStartEvent, Flow, Hooks, Host, NativeScreenView, ScreenMouseButtonPressedPreEvent,
    ScreenRenderPostEvent, Text,
};
use horizon_client::widgets;

const BUTTON_WIDTH: f32 = 150.0;
const BUTTON_HEIGHT: f32 = 20.0;

pub fn hooks() -> Box<dyn Hooks> {
    Box::new(PlanarWorlds)
}

struct PlanarWorlds;

impl Hooks for PlanarWorlds {
    fn client_start(&self, _host: &mut dyn Host, _event: &mut ClientStartEvent) {
        log::info!("Planar Worlds Loaded!");
    }

    fn screen_render_post(&self, host: &dyn Host, event: &mut ScreenRenderPostEvent<'_>) {
        let Some(rect) = button_rect(event.screen) else {
            return;
        };
        let over_button = hovered(rect, event.screen.mouse);
        let sprite = if over_button {
            "gui/sprites/widget/button_highlighted"
        } else {
            "gui/sprites/widget/button"
        };
        let (key, fallback) = if planar_worlds_state::enabled() {
            ("gui.planar_worlds.mode_on", "Planar Mode: ON")
        } else {
            ("gui.planar_worlds.mode_off", "Planar Mode: OFF")
        };
        let label = Text::translate_with_fallback(key, fallback, &[]).plain();
        widgets::record(&label, rect, true);
        widgets::target(rect);
        widgets::widget_cursor(over_button, true);
        let [x, y, width, height] = rect;
        host.gui_nine_slice(event.batch, sprite, rect, u32::MAX);
        host.text(
            event.batch,
            &label,
            [
                x + ((width - host.text_width(&label)) / 2.0).floor(),
                y + height / 2.0 - 4.0,
            ],
            0xffffff,
            true,
        );
    }

    fn screen_mouse_button_pressed_pre(
        &self,
        _host: &dyn Host,
        event: &mut ScreenMouseButtonPressedPreEvent<'_>,
    ) -> Flow {
        if event.button == 0
            && button_rect(event.screen).is_some_and(|r| hovered(r, event.screen.mouse))
        {
            planar_worlds_state::toggle();
            Flow::Cancel
        } else {
            Flow::Continue
        }
    }
}

fn button_rect(screen: &NativeScreenView<'_>) -> Option<[f32; 4]> {
    let [x, y, width, height] = screen.rect;
    if screen.id != "world_create" || width < BUTTON_WIDTH + 8.0 || height < 57.0 {
        return None;
    }
    // Move to the right of the form on short but wide GUI layouts.
    let (left, top) = if height >= 208.0 {
        (
            x + ((width - BUTTON_WIDTH) / 2.0).floor(),
            y + height - 57.0,
        )
    } else if width >= 626.0 {
        (x + width - BUTTON_WIDTH - 4.0, y + height - 57.0)
    } else {
        return None;
    };
    Some([left, top, BUTTON_WIDTH, BUTTON_HEIGHT])
}

fn hovered([x, y, w, h]: [f32; 4], [mx, my]: [f32; 2]) -> bool {
    (x..x + w).contains(&mx) && (y..y + h).contains(&my)
}

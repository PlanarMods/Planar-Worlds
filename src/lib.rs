use mod_api::{
    ClientHost, ClientSetupEvent, Flow, Hooks, ScreenMouseButtonPressedPreEvent,
    ScreenRenderPostEvent, ScreenView, Text,
};

pub mod plane;
pub mod state;
pub mod wall_draw;

const BUTTON_WIDTH: f32 = 150.0;
const BUTTON_HEIGHT: f32 = 20.0;

#[derive(Default)]
struct PlanarWorlds;

#[mod_api::horizon_mod]
impl Hooks for PlanarWorlds {
    fn entity_move(
        &self,
        _context: &dyn mod_api::LoopContext,
        event: &mut mod_api::EntityMoveEvent<'_>,
    ) -> Flow {
        if plane::active() {
            event.delta[2] = 0.0;
        }
        Flow::Continue
    }

    fn block_spread(
        &self,
        _host: &dyn mod_api::LevelHost,
        event: &mut mod_api::BlockSpreadEvent<'_>,
    ) -> Flow {
        if plane::active() && event.to.z != plane::layer() {
            Flow::Cancel
        } else {
            Flow::Continue
        }
    }

    /// The slice is a cross-section, so its fluids must not run: a water column would fall,
    /// level and drift through the cut and every surface would animate as if flowing.
    fn fluid_tick(
        &self,
        _host: &dyn mod_api::LevelHost,
        event: &mut mod_api::FluidTickEvent<'_>,
    ) -> Flow {
        if plane::client_slice() && in_slice(event.pos) {
            Flow::Cancel
        } else {
            Flow::Continue
        }
    }

    fn fluid_level_change(
        &self,
        _host: &dyn mod_api::LevelHost,
        event: &mut mod_api::FluidLevelChangeEvent<'_>,
    ) -> Flow {
        if plane::client_slice() && in_slice(event.pos) {
            Flow::Cancel
        } else {
            Flow::Continue
        }
    }

    fn movement_input(&self, _host: &dyn ClientHost, event: &mut mod_api::MovementInputEvent) {
        if plane::client_slice() {
            event.forward = -event.strafe;
            event.strafe = 0.0;
        }
    }
    fn client_setup(&self, _host: &mut dyn ClientHost, _event: &mut ClientSetupEvent) {
        log::info!("Planar Worlds Loaded!");
    }

    fn register_gui_layers(
        &self,
        _context: &mod_api::RegistrationContext,
        event: &mut mod_api::RegisterGuiLayersEvent<'_>,
    ) {
        wall_draw::register(event);
    }

    fn gui_layer_draw(&self, host: &dyn ClientHost, event: &mut mod_api::GuiLayerDrawEvent<'_>) {
        wall_draw::draw(host, event);
    }

    fn screen_render_post(&self, _host: &dyn ClientHost, event: &mut ScreenRenderPostEvent<'_>) {
        let Some(rect) = button_rect(event.screen) else {
            return;
        };
        let over_button = hovered(rect, event.screen.mouse);
        let key = if state::enabled() {
            "gui.planar_worlds.mode_on"
        } else {
            "gui.planar_worlds.mode_off"
        };
        let label = Text::translate(key, Vec::new());
        let _ = event.draw.widget_button(rect, &label, true, over_button);
    }

    fn screen_mouse_button_pressed_pre(
        &self,
        _host: &dyn ClientHost,
        event: &mut ScreenMouseButtonPressedPreEvent<'_>,
    ) -> Flow {
        if event.button == 0
            && button_rect(event.screen).is_some_and(|r| hovered(r, event.screen.mouse))
        {
            state::toggle();
            Flow::Cancel
        } else {
            Flow::Continue
        }
    }
}

fn button_rect(screen: &ScreenView<'_>) -> Option<[f32; 4]> {
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

/// The world Z of the slice's own blocks, from the foreground plane through the canopy layer.
fn in_slice(pos: mod_api::BlockPos) -> bool {
    (0..=crate::plane::DECOR).contains(&pos.z)
}

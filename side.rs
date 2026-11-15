use super::*;
use horizon_world::planar_plane as plane;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

thread_local! {
    static LOCAL_SLICE: Cell<bool> = const { Cell::new(false) };
    static CURSOR: Cell<Option<[f64; 2]>> = const { Cell::new(None) };
    static FREE_POINTER: Cell<bool> = const { Cell::new(false) };
    static FACING_LEFT: Cell<bool> = const { Cell::new(false) };
    static MOB_DIRECTIONS: RefCell<HashMap<u32, f32>> = RefCell::new(HashMap::new());
}

pub(super) fn select_local_slice(slice: bool) {
    LOCAL_SLICE.set(slice);
    FACING_LEFT.set(false);
    MOB_DIRECTIONS.with_borrow_mut(HashMap::clear);
}

pub(super) fn local(game: &Game<'_>) -> bool {
    LOCAL_SLICE.get() && game.integrated.is_some() && game.dim == Dim::Overworld
}

pub(super) fn server(game: &Game<'_>) -> bool {
    game.dim == Dim::Overworld && super::planar_worlds::is_slice(&game.world_root).unwrap_or(false)
}

pub(super) fn center(game: &mut Game<'_>) {
    if let Some(player) = game.player.as_mut() {
        player.position.z = plane::Z;
        player.velocity.z = 0.0;
        game.previous_position.z = plane::Z;
    }
}

pub(super) fn center_entities(game: &mut Game<'_>) {
    use super::entity::Entity;
    for handle in game.entity_handles() {
        super::entity::with_entity_mut!(game, handle, entity => {
            entity.position_mut().z = plane::Z;
            entity.velocity_mut().z = 0.0;
        });
    }
}

pub(super) fn camera(game: &Game<'_>, partial: f32) -> Option<(glam::DVec3, f32, f32)> {
    let player = game.player.as_ref()?;
    let feet = game.previous_position.add_vector(
        player
            .position
            .subtract(game.previous_position)
            .scale(f64::from(partial)),
    );
    Some((
        glam::DVec3::new(feet.x, feet.y + 2.0, plane::CAMERA_Z),
        180.0,
        0.0,
    ))
}

pub(super) fn free_pointer(game: &Game<'_>) -> bool {
    local(game) && game.keyboard_mouse
}

pub(super) fn sync_pointer(game: &mut Game<'_>) {
    let free = free_pointer(game);
    if FREE_POINTER.replace(free) == free {
        return;
    }
    if free {
        if let Some(window) = &game.window {
            let _ = window.set_cursor_grab(CursorGrabMode::None);
            window.set_cursor_visible(true);
        }
        game.capture_mode = None;
    } else if game.captured {
        game.captured = false;
        game.capture(true);
    }
}

pub(super) fn pointer_event(game: &mut Game<'_>, event: &WindowEvent) {
    if !game.keyboard_mouse {
        return;
    }
    match event {
        WindowEvent::CursorMoved { position, .. } => {
            CURSOR.set(Some([position.x, position.y]));
        }
        WindowEvent::CursorLeft { .. } => {
            CURSOR.set(None);
            if free_pointer(game) {
                game.toggles.button(2, false, false, false);
                game.toggles.button(3, false, false, false);
                game.left = false;
                game.right = false;
                game.request_remote_mining_stop();
            }
        }
        _ => {}
    }
}

pub(super) fn movement_pose(game: &mut Game<'_>) {
    game.yaw = -90.0;
    game.pitch = 0.0;
}

fn viewport(game: &Game<'_>) -> Option<super::planar_pointer::Viewport> {
    let renderer = game.renderer.as_ref()?;
    let (camera, _, _) = camera(game, game.clock.camera_fraction() as f32)?;
    super::planar_pointer::Viewport::new(
        [camera.x, camera.y],
        [renderer.config.width, renderer.config.height],
        f64::from(plane::HALF_HEIGHT),
    )
}

fn targeting(game: &Game<'_>) -> Option<super::planar_pointer::CursorRay> {
    if !game.focused
        || !gameplay_has_keyboard(
            game.screen,
            game.title.is_some(),
            game.screen_over_playing(),
        )
        || game.overlay_holds_view()
    {
        return None;
    }
    let target = viewport(game)?.screen_to_world(CURSOR.get()?)?;
    let mut eye = game.eye_position()?;
    eye[2] = plane::Z + f64::from(target_layer(game));
    super::planar_pointer::CursorRay::new(eye, target)
}

pub(super) fn target_layer(game: &Game<'_>) -> i32 {
    if game.modifiers.control_key()
        || [KeyCode::ControlLeft, KeyCode::ControlRight]
            .iter()
            .any(|key| game.physical_keys.contains(key))
    {
        plane::BACKGROUND
    } else {
        0
    }
}

pub(super) fn empty_target_hit(game: &Game<'_>) -> Option<Hit> {
    let ray = targeting(game)?;
    let [x, y] = ray.target.map(|p| p.floor() as i32);
    let z = target_layer(game);
    if ray.length > game.block_reach.min(5.0)
        || !game.world.loaded(x, z)
        || !(game.world.shape.min_y..game.world.shape.max_y()).contains(&y)
        || !game.res.facts().is_air(game.world.block(x, y, z))
    {
        return None;
    }
    if z == 0
        && let Some(player) = &game.player
    {
        let min = Vec3::new(f64::from(x), f64::from(y), 0.0);
        let max = Vec3::new(f64::from(x) + 1.0, f64::from(y) + 1.0, 1.0);
        if Aabb::new(min, max).intersects(player.bounds()) {
            return None;
        }
    }
    Some(Hit {
        pos: [x, y, z],
        face: [0, 1, 0],
        point: [ray.target[0], ray.target[1], ray.eye[2]],
        inside: false,
    })
}

pub(super) fn can_repeat_air_placement(game: &Game<'_>) -> bool {
    target_layer(game) == 0
        && game
            .held()
            .into_iter()
            .chain(game.offhand.as_ref())
            .any(|stack| {
                game.items()
                    .get(&stack.item)
                    .is_some_and(|item| item.block.is_some())
            })
        && game.pick().is_none()
        && empty_target_hit(game).is_some()
        && !game.remote_use_targets_entity()
}

pub(super) fn aim(game: &Game<'_>) -> Option<([f64; 3], [f64; 3])> {
    let ray = targeting(game)?;
    Some((ray.eye, ray.direction))
}

pub(super) fn pick(game: &Game<'_>, reach: f64) -> Option<Hit> {
    let ray = targeting(game)?;
    game.clip_ray_fluids(ray.eye, ray.direction, ray.extent(reach), false)
}

pub(super) fn pick_fluid(game: &Game<'_>) -> Option<Hit> {
    let ray = targeting(game)?;
    game.clip_ray_fluids(ray.eye, ray.direction, ray.extent(game.reach()), true)
}

pub(super) fn entity_before_cursor(game: &Game<'_>, distance: f64) -> bool {
    target_layer(game) == 0
        && targeting(game).is_some_and(|ray| distance <= ray.length)
        && game.pick_unfiltered().is_none()
}

pub(super) fn snap_model_facing(game: &mut Game<'_>) -> Option<ModelPose> {
    let target = targeting(game).map(|ray| {
        let target = game.pick().map_or(ray.target, |hit| {
            [f64::from(hit.pos[0]) + 0.5, f64::from(hit.pos[1]) + 0.5]
        });
        [target[0] - ray.eye[0], target[1] - ray.eye[1]]
    });
    let model = game.player_model.as_mut()?;
    let (yaw, pitch, left) = super::planar_pointer::model_rotation(target, FACING_LEFT.get());
    FACING_LEFT.set(left);
    Some(ModelPose::replace(model, yaw, pitch))
}

pub(super) struct ModelPose {
    yaw: [f32; 6],
    pitch: [f32; 2],
    vehicle: Option<(u32, f32, f32)>,
}

impl ModelPose {
    fn replace(model: &mut Mob, yaw: f32, pitch: f32) -> Self {
        let old = Self {
            yaw: [
                model.yaw,
                model.previous_yaw,
                model.body_yaw,
                model.previous_body_yaw,
                model.head_yaw,
                model.previous_head_yaw,
            ],
            pitch: [model.head_pitch, model.previous_head_pitch],
            vehicle: model.extra.vehicle_body,
        };
        // The renderer interpolates these pairs independently. Set both endpoints
        // so a side change never draws a front-facing intermediate frame.
        model.yaw = yaw;
        model.previous_yaw = yaw;
        model.body_yaw = yaw;
        model.previous_body_yaw = yaw;
        model.head_yaw = yaw;
        model.previous_head_yaw = yaw;
        model.head_pitch = pitch;
        model.previous_head_pitch = pitch;
        if let Some((_, previous, current)) = model.extra.vehicle_body.as_mut() {
            *previous = yaw;
            *current = yaw;
        }
        old
    }

    pub(super) fn restore(self, model: &mut Mob) {
        [
            model.yaw,
            model.previous_yaw,
            model.body_yaw,
            model.previous_body_yaw,
            model.head_yaw,
            model.previous_head_yaw,
        ] = self.yaw;
        [model.head_pitch, model.previous_head_pitch] = self.pitch;
        model.extra.vehicle_body = self.vehicle;
    }
}

pub(super) fn mob_poses(game: &mut Game<'_>) -> HashMap<u32, ModelPose> {
    MOB_DIRECTIONS.with_borrow_mut(|last| {
        let poses: HashMap<_, _> = game
            .mobs
            .iter_mut()
            .map(|mob| {
                let delta = mob.position.x - mob.previous.x;
                let direction = last.entry(mob.id).or_insert(-90.0);
                *direction = super::planar_pointer::mob_yaw(delta, mob.head_yaw, *direction);
                (mob.id, ModelPose::replace(mob, *direction, 0.0))
            })
            .collect();
        last.retain(|id, _| poses.contains_key(id));
        poses
    })
}

pub(super) fn highlight(game: &Game<'_>, batch: &mut HudBatch) {
    if !game.may_build() && game.mode != horizon_client::command::args::GameMode::Adventure {
        return;
    }
    let (Some(hit), Some(viewport)) = (game.pick(), viewport(game)) else {
        return;
    };
    if !game.may_build()
        && game.block_action_restricted(hit.pos)
        && !game.can_place_on_in_adventure(hit.pos)
    {
        return;
    }
    let [x, y, _] = hit.pos.map(f64::from);
    let [left, top] = viewport
        .world_to_screen([x, y + 1.0])
        .map(|p| p as f32 / batch.scale);
    let [right, bottom] = viewport
        .world_to_screen([x + 1.0, y])
        .map(|p| p as f32 / batch.scale);
    let foreground = if hit.pos[2] == plane::BACKGROUND {
        0xffad_b7c5
    } else {
        0xffff_ffff
    };
    for (pixels, color) in [(3.0, 0xff00_0000), (1.0, foreground)] {
        let stroke = pixels / batch.scale;
        let half = stroke * 0.5;
        let w = right - left + stroke;
        let h = bottom - top + stroke;
        let atlas = &game.gui.atlas;
        batch.fill(atlas, left - half, top - half, w, stroke, color);
        batch.fill(atlas, left - half, bottom - half, w, stroke, color);
        batch.fill(atlas, left - half, top - half, stroke, h, color);
        batch.fill(atlas, right - half, top - half, stroke, h, color);
    }
}

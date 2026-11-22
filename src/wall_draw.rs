//! Depth shading and light for the cut face, drawn screen-space by a HUD layer
//! the mod registers below every other layer (`GuiLayerPlacement::BelowAll`):
//! after the world, before the hotbar, hearts and crosshair.
//!
//! The game already meshes the background layer's own front faces (the blocks
//! at `Z=1` render like any terrain where the foreground is air or culled).
//! What they lack is the 0.3.6 look: a depth darkening under the surface and
//! light that finds its way into dug shafts and caves. Per-frame world-space
//! quads cannot overlay terrain (`submit_custom_geometry` runs before the
//! opaque pass), so this works on the screen pixels instead. Each visible
//! underground cell gets one filled rectangle: darkened by depth from the
//! surface, lifted where a per-frame light propagation reaches it. A lifted
//! (dark) cell is painted with translucent white — source-over brightening —
//! which the game-lit dark texels need; a darker one with translucent black.

use mod_api::{ClientHost, GuiLayerDrawEvent, RegisterGuiLayersEvent};
use std::collections::BinaryHeap;

/// The mod's HUD layer, below every other layer (the crosshair included).
const LAYER: &str = "planar_worlds:wall_shade";
/// Over-generated view range in blocks around the camera.
const HALF_WIDTH: i32 = 120;
const HALF_HEIGHT: i32 = 70;
/// Darkening curve: brightness = (1 - depth / FULL_DARK)^CURVE, as alpha.
/// Fitted to the 0.3.6 references: full brightness at the surface, ~70% at six
/// blocks, half at twelve, near-black by the low twenties.
const FULL_DARK: f32 = 34.0;
const CURVE: f32 = 1.8;
const MAX_ALPHA: f32 = 0.97;
/// Light propagation: sky light is 1 above the surface; moving into a neighbour
/// costs AIR_COST through air and SOLID_COST through a solid block. A dug shaft
/// stays lit down its length and lights the wall beside it.
const AIR_COST: f32 = 0.03;
const SOLID_COST: f32 = 0.10;
/// How much of a light surplus over the depth shade becomes white lift.
const WHITE_MAX: f32 = 0.85;

pub fn register(event: &mut RegisterGuiLayersEvent<'_>) {
    event.layers.push(mod_api::GuiLayer {
        id: LAYER.to_owned(),
        placement: mod_api::GuiLayerPlacement::BelowAll,
    });
}

pub fn draw(host: &dyn ClientHost, event: &mut GuiLayerDrawEvent<'_>) {
    if event.layer != LAYER || !crate::plane::client_slice() {
        return;
    }
    let Some(level) = host.level("minecraft:overworld") else {
        return;
    };
    let Some(camera) = host.camera() else {
        return;
    };
    let air = host.block_state_id("minecraft:air");
    let [w, h] = event.draw.size();
    let (cx, cy) = (camera.position[0].floor(), camera.position[1].floor());
    // The side camera has no roll: one block step projects to a constant pixel
    // step in x and in y. Anchor the grid with three projections, then place
    // every cell arithmetically.
    let Some(origin) = pixel(host, w, h, [cx, cy, 0.0]) else {
        return;
    };
    let Some(right) = pixel(host, w, h, [cx + 1.0, cy, 0.0]) else {
        return;
    };
    let Some(up) = pixel(host, w, h, [cx, cy + 1.0, 0.0]) else {
        return;
    };
    let step_x = [right[0] - origin[0], right[1] - origin[1]];
    let step_y = [up[0] - origin[0], up[1] - origin[1]];

    // Pass 1: the slice's cells — solid foreground (z=0), the wall behind it
    // (z=1) — and each column's surface, the topmost solid foreground block.
    let (x0, y0) = (cx as i32 - HALF_WIDTH, cy as i32 - HALF_HEIGHT);
    let (gw, gh) = (2 * HALF_WIDTH as usize + 1, 2 * HALF_HEIGHT as usize + 1);
    let mut solid = vec![false; gw * gh];
    let mut wall = vec![false; gw * gh];
    let mut surface: Vec<Option<i32>> = vec![None; gw];
    for (i, x) in (x0..=x0 + gw as i32 - 1).enumerate() {
        for (j, y) in (y0..=y0 + gh as i32 - 1).enumerate() {
            let front = level.block_state(host, [x, y, 0]);
            let filled = level.block_state(host, [x, y, 1]);
            solid[i * gh + j] = differs(front, air);
            wall[i * gh + j] = differs(filled, air);
        }
        // The surface is the topmost solid foreground block.
        for j in (0..gh).rev() {
            if solid[i * gh + j] {
                surface[i] = Some(y0 + j as i32);
                break;
            }
        }
    }

    // Pass 2: light. Every cell above a column's surface is open sky at 1.0;
    // the brightest path wins (a max-heap Dijkstra, f32 bits order for x >= 0).
    let mut light = vec![0.0_f32; gw * gh];
    let mut heap: BinaryHeap<(u32, usize)> = BinaryHeap::new();
    for (i, top) in surface.iter().enumerate() {
        if let Some(top) = top {
            for j in (*top - y0 + 1).max(0) as usize..gh {
                let at = i * gh + j;
                light[at] = 1.0;
                heap.push((1.0f32.to_bits(), at));
            }
        }
    }
    while let Some((bits, at)) = heap.pop() {
        let l = f32::from_bits(bits);
        if l < light[at] - f32::EPSILON {
            continue; // a brighter path reached the cell first
        }
        let (i, j) = (at / gh, at % gh);
        for (di, dj) in [(1_i32, 0), (-1, 0), (0, 1), (0, -1)] {
            let (ni, nj) = (i as i32 + di, j as i32 + dj);
            if !(0..gw as i32).contains(&ni) || !(0..gh as i32).contains(&nj) {
                continue;
            }
            let next = (ni as usize) * gh + nj as usize;
            let cost = if solid[next] { SOLID_COST } else { AIR_COST };
            let nl = l - cost;
            if nl > 0.0 && nl > light[next] {
                light[next] = nl;
                heap.push((nl.to_bits(), next));
            }
        }
    }

    // Pass 3: one rectangle per visible solid cell. Depth below the column's
    // surface darkens; propagated light lifts, as translucent white.
    for (i, x) in (x0..=x0 + gw as i32 - 1).enumerate() {
        let Some(top) = surface[i] else {
            continue;
        };
        if top < y0 {
            continue;
        }
        for j in 0..=(top - y0) as usize {
            let at = i * gh + j;
            if !solid[at] && !wall[at] {
                continue;
            }
            let y = y0 + j as i32;
            let depth = (top - y + 1).max(0) as f32;
            let base = (1.0 - depth / FULL_DARK).max(0.0).powf(CURVE);
            let lit = light[at];
            let (argb, color) = if lit > base {
                ((lit - base) * WHITE_MAX, 0xFF_FFFF)
            } else {
                // The texel underneath renders full-bright; to land on `base`,
                // the black fill's alpha is 1 - base (as in 0.3.6).
                ((1.0 - base).min(MAX_ALPHA), 0x0000_00)
            };
            let alpha = argb.min(MAX_ALPHA);
            if alpha < 0.02 {
                continue;
            }
            let color = (((alpha * 255.0) as u32) << 24) | color;
            let px = origin[0] + (x as f64 - cx) as f32 * step_x[0] + (y as f64 - cy) as f32 * step_y[0];
            let py = origin[1] + (x as f64 - cx) as f32 * step_x[1] + (y as f64 - cy) as f32 * step_y[1];
            let rect = [
                px,
                py - step_y[1].abs(),
                step_x[0].abs(),
                step_y[1].abs(),
            ];
            if rect[0] + rect[2] < 0.0 || rect[0] > w || rect[1] + rect[3] < 0.0 || rect[1] > h {
                continue;
            }
            event.draw.fill(rect, color);
        }
    }
}

fn differs(state: Option<mod_api::BlockState>, air: Option<mod_api::BlockState>) -> bool {
    state.zip(air).map(|(state, air)| state != air).unwrap_or(false)
}

fn pixel(host: &dyn ClientHost, w: f32, h: f32, at: [f64; 3]) -> Option<[f32; 2]> {
    let [x, y, depth] = host.project(at)?;
    if depth < 0.0 {
        return None;
    }
    Some([((x + 1.0) / 2.0) * w, ((1.0 - y) / 2.0) * h])
}

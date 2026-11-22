use horizon_client::{chunk::Chunk, light::LightTables, mesher::{SectionMesh, TerrainVertex}};
use std::collections::VecDeque;

const WIDTH: usize = 48;
const SOLID_FALLOFF: u8 = 4;
const EXPOSURE_BOOST: u8 = 1;
const STEPS: [(isize, isize, usize); 4] = [(0, -1, 0), (0, 1, 1), (-1, 0, 4), (1, 0, 5)];

pub(crate) fn shade(
    mesh: &mut SectionMesh,
    region: &[Option<&Chunk>; 3],
    sy: i32,
    tables: &LightTables,
) {
    if mesh.solid.is_empty() && mesh.translucent.is_empty() {
        return;
    }
    let center = region[1].expect("center chunk");
    let height = (center.max_y() - center.min_y()) as usize;
    let mut present = [false; WIDTH];
    let mut cells = vec![0; WIDTH * height];
    let mut sky = vec![0; cells.len()];
    let mut block = vec![0; cells.len()];
    for (column, chunk) in region.iter().enumerate() {
        let Some(chunk) = chunk else { continue };
        for x in 0..16 {
            let cx = column * 16 + x;
            present[cx] = true;
            let mut open = true;
            for y in (0..height).rev() {
                let state = chunk.get(x as i32, center.min_y() + y as i32, 0) as usize;
                let at = y * WIDTH + cx;
                cells[at] = u16::from(tables.dampening(state as u32));
                block[at] = tables.emission(state as u32);
                open &= cells[at] == 0;
                if open {
                    sky[at] = 15;
                }
            }
        }
    }
    spread(&present, &cells, &mut sky);
    spread(&present, &cells, &mut block);
    for x in 0..WIDTH {
        let at = (height - 1) * WIDTH + x;
        if present[x] && cells[at] & 15 == 15 {
            sky[at] = 15 - SOLID_FALLOFF;
        }
    }
    shade_solid_layers(&present, &cells, &mut sky);
    shade_solid_layers(&present, &cells, &mut block);

    let front_light = |quad: &[TerrainVertex]| {
        let x = ((quad.iter().map(|v| v.pos[0]).sum::<f32>() * 0.25).floor() as i32).clamp(0, 15);
        let y = ((quad.iter().map(|v| v.pos[1]).sum::<f32>() * 0.25).floor() as i32).clamp(0, 15);
        let at = (sy as usize * 16 + y as usize) * WIDTH + 16 + x as usize;
        (u16::from(display_light(sky[at])) << 4) | (u16::from(display_light(block[at])) << 12)
    };

    let apply = |quad: &mut [TerrainVertex], light: u16| {
        for vertex in quad {
            vertex.light = light;
            if light == 0 {
                vertex.color = 0xff00_0000;
            }
        }
    };
    for (i, quad) in mesh.solid.chunks_exact_mut(4).enumerate() {
        let light = front_light(quad);
        if light == 0
            && let Some(tint) = mesh.solid_tints.get_mut(i)
        {
            *tint = 0;
        }
        apply(quad, light);
    }
    for quad in mesh.translucent.chunks_exact_mut(4) {
        let light = front_light(quad);
        apply(quad, light);
    }
}

fn display_light(level: u8) -> u8 {
    if level == 0 {
        0
    } else {
        (level + EXPOSURE_BOOST).min(15)
    }
}

// Visual falloff for the cut faces. This pass only writes into opaque cells,
// so it cannot carry light through a wall into a different cave.
fn shade_solid_layers(present: &[bool; WIDTH], cells: &[u16], levels: &mut [u8]) {
    let height = cells.len() / WIDTH;
    let mut queue: VecDeque<_> = levels
        .iter()
        .enumerate()
        .filter_map(|(at, &value)| (value > SOLID_FALLOFF).then_some(at))
        .collect();
    while let Some(at) = queue.pop_front() {
        let candidate = levels[at] - SOLID_FALLOFF;
        for (next, _) in neighbors(at, height, present) {
            if cells[next] & 15 == 15 && candidate > levels[next] {
                levels[next] = candidate;
                if candidate > SOLID_FALLOFF {
                    queue.push_back(next);
                }
            }
        }
    }
}

fn neighbors(
    at: usize,
    height: usize,
    present: &[bool; WIDTH],
) -> impl Iterator<Item = (usize, usize)> + '_ {
    let x = (at % WIDTH) as isize;
    let y = (at / WIDTH) as isize;
    STEPS.into_iter().filter_map(move |(dx, dy, dir)| {
        let (x, y) = (x + dx, y + dy);
        if (0..WIDTH as isize).contains(&x)
            && (0..height as isize).contains(&y)
            && present[x as usize]
        {
            Some((y as usize * WIDTH + x as usize, dir))
        } else {
            None
        }
    })
}

fn spread(present: &[bool; WIDTH], cells: &[u16], levels: &mut [u8]) {
    let height = cells.len() / WIDTH;
    let mut queue: VecDeque<_> = levels
        .iter()
        .enumerate()
        .filter_map(|(at, &value)| (value > 1).then_some(at))
        .collect();
    while let Some(at) = queue.pop_front() {
        let value = levels[at];
        for (next, _) in neighbors(at, height, present) {
            let candidate = value.saturating_sub((cells[next] as u8 & 15).max(1));
            if candidate > levels[next] && cells[next] < 15
            {
                levels[next] = candidate;
                if candidate > 1 {
                    queue.push_back(next);
                }
            }
        }
    }
}

use super::*;
use mod_planar_worlds::plane;
use std::cell::Cell;

thread_local! {
    static HIDDEN: Cell<bool> = const { Cell::new(false) };
}

pub(super) fn hidden() -> bool {
    HIDDEN.get()
}

/// Give physics a foreground-only view and restore background cells before returning.
/// Foreground writes performed during the tick survive, including on an error.
pub(super) fn run<'a, T>(game: &mut Game<'a>, tick: impl FnOnce(&mut Game<'a>) -> T) -> T {
    let saved: Vec<_> = game.world.chunks.iter()
        .filter(|((_, z), _)| *z == 0)
        .map(|(&key, chunk)| (key, chunk.clone()))
        .collect();
    for (key, _) in &saved {
        let chunk = Arc::make_mut(game.world.chunks.get_mut(key).expect("loaded column"));
        for sy in 0..chunk.sections.len() {
            let mut blocks = [0; 4096];
            chunk.sections[sy].unpack(&mut blocks);
            for row in blocks.chunks_exact_mut(256) {
                row[plane::BACKGROUND as usize * 16..(plane::BACKGROUND as usize + 1) * 16].fill(0);
            }
            chunk.replace_blocks(sy, &blocks);
        }
    }
    HIDDEN.set(true);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tick(game)));
    HIDDEN.set(false);
    for (key, old) in saved {
        let Some(current) = game.world.chunks.get_mut(&key) else { continue };
        let chunk = Arc::make_mut(current);
        for sy in 0..chunk.sections.len().min(old.sections.len()) {
            let mut blocks = [0; 4096];
            let mut original = [0; 4096];
            chunk.sections[sy].unpack(&mut blocks);
            old.sections[sy].unpack(&mut original);
            for (row, source) in blocks.chunks_exact_mut(256).zip(original.chunks_exact(256)) {
                let range = plane::BACKGROUND as usize * 16..(plane::BACKGROUND as usize + 1) * 16;
                row[range.clone()].copy_from_slice(&source[range]);
            }
            chunk.replace_blocks(sy, &blocks);
        }
    }
    match result {
        Ok(value) => value,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

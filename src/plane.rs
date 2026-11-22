use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};

static CLIENT_SLICE: AtomicBool = AtomicBool::new(false);

pub fn set_client_slice(active: bool) {
    CLIENT_SLICE.store(active, Ordering::Relaxed);
}

pub fn client_slice() -> bool {
    CLIENT_SLICE.load(Ordering::Relaxed)
}

pub const Z: f64 = 0.5;
pub const BACKGROUND: i32 = 1;
/// The tree canopy's own layer: in front of the background trunk, out of the background layer
/// so the trunk stays directly minable and the foliage is never a background block.
pub const DECOR: i32 = 2;
pub const CAMERA_Z: f64 = 24.5;
pub const BACKGROUND_DEPTH_SHIFT: f32 = 2.0625;
pub const HALF_HEIGHT: f32 = 12.0;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static LAYER: Cell<i32> = const { Cell::new(0) };
    static GENERATING: Cell<bool> = const { Cell::new(false) };
    static COMBO_BREAK: Cell<bool> = const { Cell::new(false) };
}

pub fn active() -> bool {
    ACTIVE.get()
}

pub fn layer() -> i32 {
    LAYER.get()
}

pub fn generating() -> bool {
    GENERATING.get()
}

pub fn generate<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) {
            GENERATING.set(self.0);
        }
    }
    let _reset = Reset(GENERATING.replace(true));
    run()
}

/// X/Y offsets and leaf distance. A missing distance denotes vertical oak log.
pub fn tree_layout(height: i32) -> impl Iterator<Item = ([i32; 2], Option<u8>)> {
    (0..height).map(|y| ([0, y], None)).chain(
        [
            (height - 2, 2_i32),
            (height - 1, 2),
            (height, 1),
            (height + 1, 0),
        ]
        .into_iter()
        .flat_map(move |(y, radius)| {
            (-radius..=radius)
                .filter(move |&x| x != 0 || y >= height)
                .map(move |x| ([x, y], Some((x.abs() + (y - height + 1).max(0)) as u8)))
        }),
    )
}

pub fn background<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(i32);
    impl Drop for Reset {
        fn drop(&mut self) {
            LAYER.set(self.0);
        }
    }
    let _reset = Reset(LAYER.replace(BACKGROUND));
    run()
}

/// One dig clears the whole cell: while set, the foreground-break mixin lets the
/// game's own break run instead of mirroring the break onto the background layer.
pub fn combo_break<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) {
            COMBO_BREAK.set(self.0);
        }
    }
    let _reset = Reset(COMBO_BREAK.replace(true));
    run()
}

pub fn in_combo_break() -> bool {
    COMBO_BREAK.get()
}

pub fn scoped<T>(run: impl FnOnce() -> T) -> T {
    scope(true, run)
}

pub fn outside<T>(run: impl FnOnce() -> T) -> T {
    scope(false, run)
}

fn scope<T>(active: bool, run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) {
            ACTIVE.set(self.0);
        }
    }

    let _reset = Reset(ACTIVE.replace(active));
    run()
}

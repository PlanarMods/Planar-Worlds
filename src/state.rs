use std::sync::atomic::{AtomicBool, Ordering};

static PLANAR_MODE: AtomicBool = AtomicBool::new(false);

pub fn enabled() -> bool {
    PLANAR_MODE.load(Ordering::Relaxed)
}

pub fn toggle() {
    PLANAR_MODE.fetch_xor(true, Ordering::Relaxed);
}

pub fn take_selection() -> bool {
    PLANAR_MODE.swap(false, Ordering::Relaxed)
}

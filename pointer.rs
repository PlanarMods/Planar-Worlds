pub(super) struct Viewport {
    center: [f64; 2],
    size: [f64; 2],
    units_per_pixel: f64,
}

impl Viewport {
    pub(super) fn new(center: [f64; 2], size: [u32; 2], half_height: f64) -> Option<Self> {
        if size.contains(&0) {
            return None;
        }
        let size = size.map(f64::from);
        Some(Self {
            center,
            size,
            units_per_pixel: 2.0 * half_height / size[1],
        })
    }

    pub(super) fn screen_to_world(&self, [x, y]: [f64; 2]) -> Option<[f64; 2]> {
        if !(0.0..self.size[0]).contains(&x) || !(0.0..self.size[1]).contains(&y) {
            return None;
        }
        Some([
            self.center[0] + (x - self.size[0] * 0.5) * self.units_per_pixel,
            self.center[1] - (y - self.size[1] * 0.5) * self.units_per_pixel,
        ])
    }

    pub(super) fn world_to_screen(&self, [x, y]: [f64; 2]) -> [f64; 2] {
        [
            self.size[0] * 0.5 + (x - self.center[0]) / self.units_per_pixel,
            self.size[1] * 0.5 - (y - self.center[1]) / self.units_per_pixel,
        ]
    }
}

pub(super) struct CursorRay {
    pub(super) eye: [f64; 3],
    pub(super) target: [f64; 2],
    pub(super) direction: [f64; 3],
    pub(super) length: f64,
}

impl CursorRay {
    pub(super) fn new(eye: [f64; 3], target: [f64; 2]) -> Option<Self> {
        let [dx, dy] = [target[0] - eye[0], target[1] - eye[1]];
        let length = dx.hypot(dy);
        if !length.is_finite() || length <= f64::EPSILON {
            return None;
        }
        Some(Self {
            eye,
            target,
            direction: [dx / length, dy / length, 0.0],
            length,
        })
    }

    pub(super) fn extent(&self, reach: f64) -> f64 {
        self.length.min(reach)
    }
}

pub(super) fn model_rotation(delta: Option<[f64; 2]>, left: bool) -> (f32, f32, bool) {
    let [dx, dy] = delta.unwrap_or([0.0, 0.0]);
    let left = if dx.abs() > 1e-6 { dx < 0.0 } else { left };
    let yaw = if left { 90.0 } else { -90.0 };
    let pitch = (-dy.atan2(dx.abs())).to_degrees() as f32;
    (yaw, pitch, left)
}

pub(super) fn mob_yaw(moved_x: f64, yaw: f32, last: f32) -> f32 {
    let look_x = f64::from(-yaw.to_radians().sin());
    let x = if moved_x.abs() > 1.0e-4 {
        moved_x
    } else if look_x.abs() > 0.05 {
        look_x
    } else {
        return last;
    };
    if x < 0.0 { 90.0 } else { -90.0 }
}

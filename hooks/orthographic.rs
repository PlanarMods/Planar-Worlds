fn orthographic(fov_degrees: f32, aspect: f32, far: f32) -> ::core::ops::ControlFlow<Mat4> {
    if fov_degrees >= 0.0 {
        return std::ops::ControlFlow::Continue(());
    }
    let half_y = -fov_degrees;
    let half_x = half_y * aspect;
    std::ops::ControlFlow::Break(Mat4::orthographic_rh(
        -half_x, half_x, -half_y, half_y, far, NEAR_PLANE,
    ))
}

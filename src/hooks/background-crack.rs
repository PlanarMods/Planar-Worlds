fn background_crack(
    model: &BakedModel,
    mut origin: glam::Vec3,
    translucent: bool,
    stage: [f32; 2],
    out: &mut Vec<Vertex>,
) -> ::core::ops::ControlFlow<()> {
    use mod_planar_worlds::plane as plane;
    let z = f64::from(origin.z) + plane::CAMERA_Z;
    if plane::active() && z.round() == f64::from(plane::BACKGROUND) {
        origin.z -= plane::BACKGROUND_DEPTH_SHIFT;
        geometry(model, origin, translucent, stage, out);
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

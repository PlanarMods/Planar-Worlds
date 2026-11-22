fn planar_crack_origin(mut origin: glam::Vec3) -> glam::Vec3 {
    use mod_planar_worlds::plane;
    let z = f64::from(origin.z) + plane::CAMERA_Z;
    if plane::active() && z.round() == f64::from(plane::BACKGROUND) {
        origin.z -= plane::BACKGROUND_DEPTH_SHIFT;
    }
    origin
}

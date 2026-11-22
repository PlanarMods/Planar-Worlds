fn side_world_write(z: i32) -> ::core::ops::ControlFlow<crate::Result<()>> {
    use mod_planar_worlds::plane as plane;
    if plane::active() && z != plane::layer() {
        std::ops::ControlFlow::Break(Err(
            "Planar Worlds: block is outside the active slice".into()
        ))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

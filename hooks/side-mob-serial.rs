fn side_mob_serial() -> ::core::ops::ControlFlow<Option<Plan>> {
    if horizon_world::planar_plane::active() {
        std::ops::ControlFlow::Break(None)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

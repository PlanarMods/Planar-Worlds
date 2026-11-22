fn side_mob_serial() -> ::core::ops::ControlFlow<Option<Plan>> {
    if mod_planar_worlds::plane::active() {
        std::ops::ControlFlow::Break(None)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

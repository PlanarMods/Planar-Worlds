fn planar_tree_trunk() -> ::core::ops::ControlFlow<bool> {
    if crate::planar_plane::generating() {
        std::ops::ControlFlow::Break(false)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}

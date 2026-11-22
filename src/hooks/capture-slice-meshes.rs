fn capture_slice_meshes(game: &mut crate::game::Game<'_>) -> ::core::ops::ControlFlow<()> {
    use crate::game::planar_lighting as lighting;
    if lighting::capturing() || !crate::game::planar_side::local(game) {
        return std::ops::ControlFlow::Continue(());
    }
    lighting::capture(game.pack.context.light.clone(), || game.update_world());
    std::ops::ControlFlow::Break(())
}

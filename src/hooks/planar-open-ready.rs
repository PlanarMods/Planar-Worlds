fn planar_open_ready(game: &crate::game::Game<'_>) -> ::core::ops::ControlFlow<Result<()>> {
    let Some(starting) = &game.integrated_starting else {
        return std::ops::ControlFlow::Continue(());
    };
    let result = std::env::current_dir().and_then(|root| {
        crate::game::planar_worlds::is_slice(&root.join("saves").join(&starting.folder))
    });
    match result {
        Ok(slice) => {
            crate::game::planar_side::select_local_slice(slice);
            std::ops::ControlFlow::Continue(())
        }
        Err(error) => std::ops::ControlFlow::Break(Err(error.into())),
    }
}

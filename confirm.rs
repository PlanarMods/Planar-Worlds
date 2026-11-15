fn confirm(
    game: &crate::game::Game<'_>,
    folder: &str,
    create: Option<&crate::title::WorldOptions>,
) -> ::core::ops::ControlFlow<Result<()>> {
    use std::ops::ControlFlow::{Break, Continue};
    let result = (|| -> Result<bool> {
        let root = std::env::current_dir()?.join("saves").join(folder);
        if create.is_some() && game.native_on("planar_worlds") && planar_worlds_state::enabled() {
            crate::game::planar_worlds::mark_new_world(folder)?;
            planar_worlds_state::take_selection();
            log::info!("Planar Worlds: Planar Mode ON for world {folder:?}");
        }
        Ok(crate::game::planar_worlds::is_slice(&root)?)
    })();
    match result {
        Ok(slice) => {
            crate::game::planar_side::select_local_slice(slice);
            Continue(())
        }
        Err(error) => Break(Err(error)),
    }
}

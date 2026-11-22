fn anchor_spawn(
    game: &crate::game::Game<'_>,
) -> ::core::ops::ControlFlow<Result<((i32, i32), i32)>> {
    use std::ops::ControlFlow::{Break, Continue};

    if game.dim != crate::game::Dim::Overworld {
        return Continue(());
    }
    match crate::game::planar_worlds::is_planar(&game.world_root) {
        Ok(false) => Continue(()),
        Ok(true) => {
            if crate::game::planar_side::server(game) {
                let x = game
                    .end
                    .locator()
                    .and_then(|source| source.origin().ok())
                    .map_or(0, |origin| origin.x);
                return Break(Ok(((x, 0), 64)));
            }
            Break(Ok((
                (0, 0),
                game.level_shapes.overworld.min_y + crate::game::planar_worlds::LAYER_COUNT,
            )))
        }
        Err(error) => Break(Err(error.into())),
    }
}

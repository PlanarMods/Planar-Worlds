fn select_generator(
    res: &crate::game::Resources,
    pack: &crate::game::PackResources,
    options: &crate::game::Options,
    settings: &crate::game::Settings,
    world_root: &std::path::Path,
    dim: crate::game::Dim,
    shape: crate::chunk::LevelShape,
    seed: i64,
    enabled_features: &[String],
) -> ::core::ops::ControlFlow<Result<(Streaming, Arc<SaveDir>, Option<Arc<VanillaTerrain>>)>> {
    use std::ops::ControlFlow::{Break, Continue};

    if dim != crate::game::Dim::Overworld {
        return Continue(());
    }
    match crate::game::planar_worlds::is_planar(world_root) {
        Ok(false) => Continue(()),
        Ok(true) => Break(crate::game::planar_worlds::make_planar_streaming(
            res,
            pack,
            options,
            settings,
            world_root,
            shape,
            seed,
            enabled_features,
        )),
        Err(error) => Break(Err(error.into())),
    }
}

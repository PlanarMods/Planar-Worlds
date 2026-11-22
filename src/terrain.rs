use super::planar_trees::{PlanarTrees, without_vanilla_trees};
use super::{Options, PackResources, Resources, Settings};
use crate::{
    Result,
    chunk::{BIOME_STRATEGY, Chunk, LevelShape},
    lod::Surface,
    save::{Persistent, SaveDir},
    streaming::{ChunkGenerator, Proto, Streaming},
    vanillagen::VanillaTerrain,
};
use horizon_core::{
    LevelKey,
    biome::{BiomeZoom, obfuscate_seed},
    paletted::PalettedContainer,
};
use mod_planar_worlds::plane as plane;
use horizon_world::registry::BlockStateId;
use horizon_world::worldgen::generator::GeneratorWorker;
use std::{fs, io, path::Path, sync::Arc};

const MARKER: &str = "planar-worlds-v2";
const WIDTH: i32 = 3;
pub(super) const LAYER_COUNT: i32 = 4;

pub(super) fn is_planar(root: &Path) -> io::Result<bool> {
    Ok(is_slice(root)? || marker_exists(root, "planar-worlds-v1")?)
}

pub(super) fn is_slice(root: &Path) -> io::Result<bool> {
    marker_exists(root, MARKER)
}

fn marker_exists(root: &Path, marker: &str) -> io::Result<bool> {
    if root.as_os_str().is_empty() {
        return Ok(false);
    }
    match fs::symlink_metadata(root.join(marker)) {
        Ok(metadata) if metadata.is_file() => Ok(true),
        Ok(_) => Err(io::Error::other(
            "Planar Worlds save marker must be a regular file",
        )),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}

pub(super) fn mark_new_world(folder: &str) -> Result<()> {
    let root = std::env::current_dir()?.join("saves").join(folder);
    if root.join("level.dat").try_exists()? || root.join("level.dat_old").try_exists()? {
        return Err(format!("Planar Worlds: {folder:?} is already a saved world").into());
    }
    fs::create_dir_all(&root)?;
    // Creating the empty file atomically is enough to commit the generator choice.
    // Let a new-world creation retry reuse its marker after a startup failure.
    match fs::File::create_new(root.join(MARKER)) {
        Ok(_) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists && is_slice(&root)? => Ok(()),
        Err(error) => Err(error.into()),
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn make_planar_streaming(
    res: &Resources,
    pack: &PackResources,
    options: &Options,
    settings: &Settings,
    world_root: &Path,
    shape: LevelShape,
    seed: i64,
    _enabled_features: &[String],
    biomes: Arc<horizon_client::biome::Biomes>,
) -> Result<(Streaming, Arc<SaveDir>, Option<Arc<VanillaTerrain>>)> {
    let registry = pack.assets.registry;
    let block = |name: &str| {
        registry
            .default_state(name)
            .ok_or_else(|| format!("Planar Worlds: missing block {name}"))
    };
    let dirt = block("minecraft:dirt")?;
    let slice = is_slice(world_root)?;
    let vanilla = if slice {
        let dir = options.resources.join("worldgen");
        let overrides = without_vanilla_trees(&dir)?;
        Some(Arc::new(plane::generate(|| {
            VanillaTerrain::for_world_with(
                horizon_client::vanillagen::TerrainResources {
                    data: Some(&pack.data_changes),
                    dir: &dir,
                    registry,
                    biomes: &biomes,
                    item_data: &res.core.item_data,
                },
                &overrides,
                seed,
                "minecraft:normal",
                "minecraft:overworld",
                false,
            )
        })?))
    } else {
        None
    };
    let terrain = PlanarTerrain {
        shape,
        vanilla: vanilla.clone(),
        trees: slice
            .then(|| PlanarTrees::new(seed, registry))
            .transpose()?,
        air: res.facts().air.0,
        biome: biomes.id("minecraft:plains"),
        layers: [
            block("minecraft:bedrock")?,
            dirt,
            dirt,
            block("minecraft:grass_block")?,
        ],
    };
    let save = Arc::new(SaveDir::new(world_root.to_owned(), shape));
    let mut streaming = Streaming::new(
        Arc::new(Persistent {
            inner: Arc::new(terrain),
            save: save.clone(),
            registry: pack.assets.clone(),
            biomes: biomes.clone(),
        }),
        pack.context_with(biomes),
        res.threads(),
    );

    horizon_world::level::register_sea_level(
        LevelKey::builtin_or_overworld("minecraft:overworld"),
        vanilla
            .as_ref()
            .map_or(shape.min_y + 1, |source| source.sea_level()),
    );
    streaming.forced = save.load_forced_chunks();
    streaming.radius = settings.render_distance;
    streaming.biome_blend = settings.biome_blend;
    streaming.smooth_lighting = settings.smooth_lighting;
    streaming.cutout_leaves = settings.cutout_leaves;
    streaming.opaque_ice = settings.horizon.opaque_ice;
    streaming.leaf_culling = settings
        .horizon
        .leaf_culling_effective(&settings.resource_packs);
    streaming.biome_zoom = BiomeZoom::new(obfuscate_seed(seed));
    streaming.sky_light = shape.sky_light;
    streaming.mesh = options.execution_role.is_client();
    Ok((streaming, save, vanilla))
}

struct PlanarTerrain {
    shape: LevelShape,
    vanilla: Option<Arc<VanillaTerrain>>,
    trees: Option<PlanarTrees>,
    air: u32,
    biome: u16,
    layers: [BlockStateId; LAYER_COUNT as usize],
}

impl ChunkGenerator for PlanarTerrain {
    type Worker = Option<GeneratorWorker>;

    fn worker(&self) -> Self::Worker {
        self.vanilla.as_ref().map(|source| source.worker())
    }

    fn lod_surface(&self, worker: &mut Self::Worker, x: i32, z: i32) -> Option<Surface> {
        if let (Some(source), Some(worker)) = (&self.vanilla, worker) {
            return (z == 0).then(|| source.lod_surface(worker, x, z)).flatten();
        }
        (0..WIDTH).contains(&z).then_some(Surface {
            height: self.shape.min_y + LAYER_COUNT,
            state: self.layers[self.layers.len() - 1],
            water: None,
            biome: self.biome,
        })
    }

    fn proto(&self, worker: &mut Self::Worker, cx: i32, cz: i32) -> Proto {
        if let (Some(source), Some(worker)) = (&self.vanilla, worker) {
            if cz != 0 {
                return Box::new(Chunk::with_layout(cx, cz, self.shape.column()));
            }
            let mut neighbors = std::array::from_fn(|i| {
                source.proto(worker, cx + (i % 3) as i32 - 1, (i / 3) as i32 - 1)
            });
            plane::generate(|| source.decorate(worker, &mut neighbors));
            let mut chunk = source.finish(std::mem::replace(&mut neighbors[4], Box::new(())));
            for section in 0..chunk.sections.len() {
                let mut blocks = [self.air; 4096];
                let base = chunk.min_y() + 16 * section as i32;
                for y in 0..16 {
                    for x in 0..16 {
                        let state = chunk.block(x as i32, base + y as i32, 0);
                        if self
                            .trees
                            .as_ref()
                            .is_none_or(|trees| !trees.is_vanilla_tree(state))
                        {
                            blocks[y * 256 + x] = state.0;
                        }
                    }
                }
                chunk.replace_blocks(section, &blocks);
            }
            chunk.block_entities.retain(|(pos, _)| pos.2 == 0);
            chunk.block_names.retain(|(pos, _)| pos.2 == 0);
            chunk.postprocess.retain(|pos| pos[2] == 0);
            chunk.entities.clear();
            chunk.saved = None;
            if let Some(trees) = &self.trees {
                trees.decorate(&mut chunk);
            }
            return Box::new(chunk);
        }
        // Minecraft still loads and saves normal 16x16 chunks. Solid terrain occupies the
        // foreground plane and the background wall only; the canopy's layer above them stays
        // empty until a tree writes its leaves there.
        let mut chunk = Chunk::with_layout(cx, cz, self.shape.column());
        for section in &mut chunk.sections {
            Arc::make_mut(section).biomes =
                PalettedContainer::new(BIOME_STRATEGY, u32::from(self.biome));
        }
        if cz == 0 {
            for (offset, state) in self.layers.iter().enumerate() {
                let y = self.shape.min_y + offset as i32;
                for z in [0, plane::BACKGROUND] {
                    for x in 0..16 {
                        chunk.set(x, y, z, state.0);
                    }
                }
            }
        }
        Box::new(chunk)
    }

    fn light_view(&self, proto: &Proto) -> Option<Chunk> {
        proto.downcast_ref::<Chunk>().cloned()
    }

    fn finish(&self, proto: Proto) -> Chunk {
        *proto
            .downcast::<Chunk>()
            .expect("Planar Worlds proto chunk")
    }
}

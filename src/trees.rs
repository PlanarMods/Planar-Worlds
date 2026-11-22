use crate::{Result, chunk::Chunk};
use horizon_core::random::{RandomSource, XoroshiroRandom};
use horizon_world::{
    registry::{BlockRegistry, BlockStateId},
    worldgen::overrides::WorldgenOverrides,
};
use mod_planar_worlds::plane;
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::Path};

pub(super) fn without_vanilla_trees(dir: &Path) -> Result<WorldgenOverrides> {
    let mut overrides = WorldgenOverrides::default();
    for bundle in ["features", "placed_features"] {
        let path = dir.join(format!("{bundle}.json"));
        let entries: BTreeMap<String, String> = serde_json::from_str(&fs::read_to_string(&path)?)?;
        for (name, text) in entries {
            let mut value: Value = serde_json::from_str(&text)?;
            if name == "minecraft:rooted_azalea_tree" {
                value = if bundle == "placed_features" {
                    serde_json::json!({"feature": no_feature(), "placement": []})
                } else {
                    no_feature()
                };
            } else if !remove_trees(&mut value) {
                continue;
            }
            overrides.set(bundle, &name, value.to_string());
        }
    }
    Ok(overrides)
}

fn no_feature() -> Value {
    serde_json::json!({"type":"minecraft:no_op","config":{}})
}

fn remove_trees(value: &mut Value) -> bool {
    if value.get("config").is_some()
        && matches!(
            value.get("type").and_then(Value::as_str),
            Some("minecraft:tree" | "tree" | "minecraft:fallen_tree" | "fallen_tree")
        )
    {
        *value = no_feature();
        return true;
    }

    // Random selectors can carry complete placed/configured features inline.
    let mut changed = false;
    match value {
        Value::Array(entries) => {
            for entry in entries {
                changed |= remove_trees(entry);
            }
        }
        Value::Object(entries) => {
            for entry in entries.values_mut() {
                changed |= remove_trees(entry);
            }
        }
        _ => {}
    }
    changed
}

pub(super) struct PlanarTrees {
    seed: i64,
    air: BlockStateId,
    soil: Vec<BlockStateId>,
    log: BlockStateId,
    leaves: [BlockStateId; 2],
    replaceable: Vec<BlockStateId>,
    double_plants: Vec<[BlockStateId; 2]>,
    vanilla_tree_states: Vec<bool>,
    fluid_surface: Vec<BlockStateId>,
}

impl PlanarTrees {
    pub(super) fn new(seed: i64, registry: &BlockRegistry) -> Result<Self> {
        let block = |name: &str| {
            registry
                .default_state(name)
                .ok_or_else(|| format!("Planar Worlds: missing block {name}"))
        };
        let default_leaves = block("minecraft:oak_leaves")?;
        let mut leaves = registry
            .state(default_leaves)
            .expect("registered oak leaves")
            .clone();
        leaves
            .properties
            .insert("persistent".into(), "false".into());
        let mut with_distance = |distance: &str| {
            leaves.properties.insert("distance".into(), distance.into());
            registry.resolve(&leaves)
        };
        let mut double_plants = Vec::new();
        for name in ["minecraft:tall_grass", "minecraft:large_fern"] {
            if let Some(plant) = registry
                .default_state(name)
                .and_then(|id| registry.state(id))
            {
                let mut plant = plant.clone();
                plant.properties.insert("half".into(), "lower".into());
                let lower = registry.resolve(&plant)?;
                plant.properties.insert("half".into(), "upper".into());
                double_plants.push([lower, registry.resolve(&plant)?]);
            }
        }
        Ok(Self {
            seed,
            air: block("minecraft:air")?,
            soil: [
                "minecraft:grass_block",
                "minecraft:podzol",
                "minecraft:mycelium",
                "minecraft:moss_block",
            ]
            .into_iter()
            .filter_map(|name| registry.default_state(name))
            .collect(),
            log: block("minecraft:oak_log")?,
            leaves: [with_distance("1")?, with_distance("2")?],
            // Columns whose surface is a fluid get no wall; every other column
            // (soil, stone, boulders) does, or its cut face sits a block behind
            // the wall plane and reads as a recessed slit.
            fluid_surface: ["minecraft:water", "minecraft:lava"]
                .into_iter()
                .filter_map(|name| registry.default_state(name))
                .collect(),
            replaceable: [
                "minecraft:air",
                "minecraft:cave_air",
                "minecraft:void_air",
                "minecraft:short_grass",
                "minecraft:fern",
                "minecraft:dead_bush",
                "minecraft:dandelion",
                "minecraft:poppy",
                "minecraft:blue_orchid",
                "minecraft:allium",
                "minecraft:azure_bluet",
                "minecraft:oxeye_daisy",
                "minecraft:cornflower",
                "minecraft:lily_of_the_valley",
                "minecraft:red_tulip",
                "minecraft:orange_tulip",
                "minecraft:white_tulip",
                "minecraft:pink_tulip",
            ]
            .into_iter()
            .filter_map(|name| registry.default_state(name))
            .chain(double_plants.iter().flatten().copied())
            .collect(),
            double_plants,
            vanilla_tree_states: (0..registry.len())
                .map(|id| {
                    registry
                        .state(BlockStateId(id as u32))
                        .is_some_and(|state| {
                            state.name.ends_with("_leaves")
                                || state.name.ends_with("_log")
                                || state.name.ends_with("_wood")
                                || state.name == "minecraft:mangrove_roots"
                        })
                })
                .collect(),
        })
    }

    pub(super) fn is_vanilla_tree(&self, state: BlockStateId) -> bool {
        self.vanilla_tree_states
            .get(state.0 as usize)
            .copied()
            .unwrap_or(false)
    }

    pub(super) fn decorate(&self, chunk: &mut Chunk) {
        // Copy the exposed surface and enough soil and rock below it for the wall's depth
        // shading to read: the deeper layers are the ones the player watches darken.
        // Trees share this non-colliding background layer with player-built walls.
        const WALL_DEPTH: i32 = 32;
        for x in 0..16 {
            let Some(surface) = (chunk.min_y()..chunk.max_y())
                .rev()
                .find(|&y| !self.replaceable.contains(&chunk.block(x, y, 0)))
            else {
                continue;
            };
            if self.fluid_surface.contains(&chunk.block(x, surface, 0)) {
                continue;
            }
            // Walk down from the surface so a cave's air can borrow the wall block
            // already placed above it: the cut reads as one continuous wall, not sky.
            for y in ((surface - WALL_DEPTH).max(chunk.min_y())..=surface).rev() {
                let state = chunk.block(x, y, 0);
                if state != self.air {
                    chunk.set(x, y, plane::BACKGROUND, state.0);
                } else if y < surface {
                    let above = chunk.block(x, y + 1, plane::BACKGROUND);
                    if above != self.air {
                        chunk.set(x, y, plane::BACKGROUND, above.0);
                    }
                }
            }
        }
        let mut random =
            XoroshiroRandom::new(self.seed ^ i64::from(chunk.x).wrapping_mul(341_873_128_712));
        const CANDIDATES: i32 = 12;
        let start = random.next_int_bound(CANDIDATES).expect("positive bound");
        let height = 4 + random.next_int_bound(2).expect("positive bound");
        // Scan every column whose two-block crown fits in the chunk.
        let sites: Vec<_> = (0..CANDIDATES)
            .filter_map(|index| {
                let x = 2 + (start + index) % CANDIDATES;
                let surface = (chunk.min_y()..chunk.max_y()).rev().find(|&y| {
                    !self
                        .replaceable
                        .contains(&chunk.block(x, y, plane::BACKGROUND))
                })?;
                self.soil
                    .contains(&chunk.block(x, surface, plane::BACKGROUND))
                    .then_some((x, surface + 1))
            })
            .collect();
        for height in (4..=height).rev() {
            if sites
                .iter()
                .any(|&(x, base)| self.try_tree(chunk, x, base, height))
            {
                return;
            }
        }
    }

    fn try_tree(&self, chunk: &mut Chunk, x: i32, base: i32, height: i32) -> bool {
        if base + height + 1 >= chunk.max_y() {
            return false;
        }
        let blocks: Vec<_> = plane::tree_layout(height)
            .map(|([dx, dy], distance)| {
                let (state, z) = match distance {
                    None => (self.log, plane::BACKGROUND),
                    Some(d) => (self.leaves[usize::from(d - 1)], plane::DECOR),
                };
                (x + dx, base + dy, state, z)
            })
            .collect();
        if blocks.iter().any(|&(x, y, _, z)| {
            !self
                .replaceable
                .contains(&chunk.block(x, y, z))
        }) {
            return false;
        }
        // When a crown or trunk replaces one half, remove its matching half too.
        for &(x, y, _, _) in &blocks {
            let old = chunk.block(x, y, plane::BACKGROUND);
            for &[lower, upper] in &self.double_plants {
                let (other_y, other) = if old == lower {
                    (y + 1, upper)
                } else if old == upper {
                    (y - 1, lower)
                } else {
                    continue;
                };
                if chunk.block(x, other_y, plane::BACKGROUND) == other {
                    chunk.set(x, other_y, plane::BACKGROUND, self.air.0);
                }
            }
        }
        for (x, y, state, z) in blocks {
            chunk.set(x, y, z, state.0);
        }
        true
    }
}

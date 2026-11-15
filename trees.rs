use crate::{Result, chunk::Chunk};
use horizon_core::random::{RandomSource, XoroshiroRandom};
use horizon_world::{
    planar_plane as plane,
    registry::{BlockRegistry, BlockStateId},
    worldgen::overrides::WorldgenOverrides,
};
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
    vanilla_leaves: Vec<bool>,
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
            vanilla_leaves: (0..registry.len())
                .map(|id| {
                    registry
                        .state(BlockStateId(id as u32))
                        .is_some_and(|state| state.name.ends_with("_leaves"))
                })
                .collect(),
        })
    }

    pub(super) fn is_vanilla_leaf(&self, state: BlockStateId) -> bool {
        self.vanilla_leaves
            .get(state.0 as usize)
            .copied()
            .unwrap_or(false)
    }

    pub(super) fn decorate(&self, chunk: &mut Chunk) {
        let mut random =
            XoroshiroRandom::new(self.seed ^ i64::from(chunk.x).wrapping_mul(341_873_128_712));
        const CANDIDATES: i32 = 12;
        let start = random.next_int_bound(CANDIDATES).expect("positive bound");
        let height = 4 + random.next_int_bound(2).expect("positive bound");
        // Scan every column whose two-block crown fits in the chunk.
        let sites: Vec<_> = (0..CANDIDATES)
            .filter_map(|index| {
                let x = 2 + (start + index) % CANDIDATES;
                let surface = (chunk.min_y()..chunk.max_y())
                    .rev()
                    .find(|&y| !self.replaceable.contains(&chunk.block(x, y, 0)))?;
                self.soil
                    .contains(&chunk.block(x, surface, 0))
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
                (
                    x + dx,
                    base + dy,
                    distance.map_or(self.log, |d| self.leaves[usize::from(d - 1)]),
                )
            })
            .collect();
        if blocks
            .iter()
            .any(|&(x, y, _)| !self.replaceable.contains(&chunk.block(x, y, 0)))
        {
            return false;
        }
        // When a crown or trunk replaces one half, remove its matching half too.
        for &(x, y, _) in &blocks {
            let old = chunk.block(x, y, 0);
            for &[lower, upper] in &self.double_plants {
                let (other_y, other) = if old == lower {
                    (y + 1, upper)
                } else if old == upper {
                    (y - 1, lower)
                } else {
                    continue;
                };
                if chunk.block(x, other_y, 0) == other {
                    chunk.set(x, other_y, 0, self.air.0);
                }
            }
        }
        for (x, y, state) in blocks {
            chunk.set(x, y, 0, state.0);
        }
        true
    }
}

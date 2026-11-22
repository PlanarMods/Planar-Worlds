# Planar Worlds 0.3.7 — build and release state

Final, 2026-11-22 (Asia/Shanghai). Migrated from `0b14f8a` to `dd67b76` (Horizon 0.2.93).

## Result

| Item | Value |
| --- | --- |
| Base nightly | `nightly-2026-11-18-dd67b76` (`dd67b76ff23e28c6d8e21859df600d31572f81a0`) |
| Native build | latest: `native-dd67b76-1a2b88bd760d` (each `native-build` supersedes; check `%APPDATA%\horizon-launcher\versions`) |
| Package | `planar_worlds-0.3.7.hmod` (repo root copy refreshed at release; the installed one is `%APPDATA%\horizon\horizon-native-mods\planar_worlds.hmod`) |
| Public key | `RWQ34YEEsas2r0KTrPEMRVqTVaXCt6J1SRh0HT4nILG6qJ2CBs0v7Qll` (new pair generated 2026-11-22; the old key's DPAPI password was unrecoverable) |
| Signature | `planar_worlds-0.3.7.hmod.minisig` verifies against the new public key |
| Enabled | `horizon-mods.txt`: `mod.planar_worlds:true` |

## Fixed in this round

### 1. Background wall and depth shading (the 0.3.6 look)

`add_section_geometry` is a registration-style hook: on a native (mixin) build the registration
lands in `horizon_patch.dll`'s copy of the mesher statics while world meshing reads
`horizon_mod_host.dll`'s copy, so the renderer is never asked (WASM builds have no patch DLL
and are unaffected — an upstream platform bug worth reporting). The wall is now drawn
screen-space instead: a HUD layer registered `BelowAll` (`src/wall_draw.rs`) fills one rect per
visible underground cell after the world and before the hotbar, darkened by
`brightness = (1 - depth / 34)^1.8` from the local surface. Block textures come from the game's
own mesher; the shading overlays them.

### 2. Continuous wall generation

`PlanarTrees::decorate` copied the foreground column into `Z=1` cell-by-cell, so every cave
became a hole in the wall (34% density, sky showing through the cut). It now walks down from
the surface and lets cave air inherit the wall block placed above it.

### 3. Recessed columns

Columns whose surface block was not "soil" (mossy-cobble boulders, stone outcrops) got no wall,
and their foreground faces sit one plane behind the wall (`Z=1` faces render at `Z=2`), reading
as a slit "set inward". The soil gate is replaced by a fluid gate: only water/lava-surfaced
columns skip the wall.

### 4. Mining clears the whole cell; dug shafts stay readable

Breaking a foreground block only removed `Z=0`, leaving the wall block standing one plane
behind (double drops, a recessed sliver). `background-server-break` now clears `Z=1` in the
same operation (with a re-entry guard in `plane.rs`), mirrored by the client-side
`background-predict-break`.

The shade layer also propagates light: sky above the surface is 1.0 and spreads 4-neighbour
(0.03/air, 0.10/solid cell, brightest path wins), so a dug shaft stays lit down its length and
lifts the wall beside it with translucent white. Depth shade is unchanged where light doesn't
reach: `alpha = 1 - (1 - depth/34)^1.8` — note the alpha is the *darkening*, not the
brightness (two regressions came from mixing these up).

### 5. Earlier session fixes (kept)

`side-server-tick` clamps every placed guest's Z (`moved wrongly` freeze) and
`ensure_land_spawn` teleports ocean spawns to land.

## Known leftovers

- Old worlds keep their sparse, pre-fix walls; only newly generated chunks get the continuous
  wall. Test world: `WallTest2`.
- `client_slice()`-guarded per-frame cost: a 241×141 cell grid of block reads plus a Dijkstra
  light pass and ~1100 fills; add a dirty-chunk cache if real machines feel it.
- The client reads stale phantom data in the `Z=2` column of worlds written by pre-canopy
  builds; nothing reads `Z=2` anymore.

## Signing state

Resolved 2026-11-22: a fresh minisign keypair replaced the old one (the old DPAPI blob no
longer decrypted the old key; old files kept as `*.old-20261122` in the signing folder). The
new password is stored DPAPI-protected at
`%LOCALAPPDATA%\PlanarMods\Signing\planar_worlds.password.dpapi`; the public key is in the
table above. `planar_worlds-0.3.7.hmod` is signed and verifies
(`minisign -V -m planar_worlds-0.3.7.hmod -p <pub>`).

**Publishing note:** the mod registry entry still carries the old public key
(`RWQdhOB6/ghLv7vRQVstRqABa89hrLOnxlzyR8WnxiSNyHHRw61rrPyb`) — update it to the new one
when pushing 0.3.7 to the registry, or signature checks will fail for players.

## Working state

Verified offscreen via `horizon-ctl`: magenta-wall acceptance test (continuous silhouette,
trees in front), shaded gradient against the 0.3.6 references, whole-cell mining (no leftover
wall block, no double drops) and lit shafts — then confirmed in-game. No debug output left,
no game process running. The launcher instance plays the latest build.

# Planar Worlds 项目交接文档

维护日期为 2026 年 11 月 18 日，时区为 Asia/Shanghai。

## 1. 项目与交付状态

Planar Worlds 是 Horizon 的原生 Rust 模组，目标是在 Minecraft 主世界中提供类似 Paper Minecraft、泰拉瑞亚的横版 2D 生存与建造体验。项目以 Horizon 原有的世界、方块、物品、挖掘和存档系统为基础，改变地形切片、摄像机、移动、鼠标交互、背景墙和剖面光照。

| 项目 | 当前记录 |
| --- | --- |
| GitHub 仓库 | https://github.com/PlanarMods/Planar-Worlds |
| 本地项目目录 | `D:/Horizon_Module/planar-worlds` |
| 官方源码目录 | `D:/Horizon_Module/horizon` |
| 模组 ID | `planar_worlds` |
| 模组版本 | `0.3.7` |
| Minecraft 版本 | `26.3` |
| 新包格式 | `.hmod` |
| 许可证 | MIT，版权归属为 2026 PlanarMods |
| 项目主入口 | `src/lib.rs` |
| 唯一模组清单 | `horizon-mod.toml` |
| 完整原生构建 | 本轮正在验证，最终结果需要回填 |
| 安装结果 | 本轮构建通过后执行，最终结果需要回填 |

本轮的目标同时包含格式迁移、旧注入替代、背景树生成、安装，以及本交接文档。项目的游戏窗口与实机体验由人类测试。终端编译、包校验和针对性回归测试由代理执行。

### 1.1 固定版本

本地官方源码已更新到 `bc07dd266c042127e7428b945c13e1ccb0f66660`。该提交的 SDK 在迁移开始时尚未发布，因此模组固定到已经发布 SDK 的 Nightly 提交 `0b14f8acfd97fa2c750e42795249dcc981178ca4`。

Mixin 工具固定到 `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`。SDK 声明的 Rust 版本为 `1.99.1`，Windows 平台使用 `x86_64-pc-windows-gnu`。启动器会管理匹配的 Rust 和 C/link 工具链。

更新 Horizon 时，需要同时检查源码、SDK、Mixin 工具、函数索引和辅助代码。仅修改清单中的提交号不足以证明兼容。

## 2. 需求与交互约定

### 2.1 空间与摄像机

玩家活动平面位于 `Z=0` 的方块层，实体中心使用 `Z=0.5`。左右移动对应世界 X 轴，上下与跳跃对应世界 Y 轴。背景层存储在 `Z=1`。

侧面摄像机位于玩家前方的固定 Z 距离，沿 X/Y 跟随玩家。鼠标位置用于选取世界坐标。玩家的视觉朝向根据目标决定，移动方向使用独立的输入映射。

| 常量 | 数值 | 用途 |
| --- | --- | --- |
| `plane::Z` | `0.5` | 实体平面中心 |
| `plane::BACKGROUND` | `1` | 背景存储层 |
| `plane::CAMERA_Z` | `24.5` | 侧面摄像机 Z 坐标 |
| `plane::HALF_HEIGHT` | `12.0` | 正交视图的垂直半高 |
| `plane::BACKGROUND_DEPTH_SHIFT` | `2.0625` | 背景网格的视觉深度偏移 |

### 2.2 操作

| 操作 | 行为 |
| --- | --- |
| 创建世界界面的 Planar Mode 按钮 | 切换新世界的生成模式 |
| A / D | 左右移动 |
| Space | 跳跃 |
| 移动鼠标 | 在侧视平面上选择目标 |
| 按住左键 | 对最终选中的前景方块持续挖掘 |
| 右键 | 正常使用或放置方块 |
| 手持方块并按住右键指向空气 | 在可达的空格中放置，距离最多五格 |
| Ctrl + 鼠标交互 | 选择、挖掘或放置背景层 |

空气放置会检查玩家占用位置，避免将方块放进玩家身体。目标选取需要先沿玩家眼睛到鼠标目标执行射线检测，途中遇到的第一个实体方块优先成为目标。

### 2.3 当前功能边界

当前玩法面向单人主世界。下界传送门、末地传送门，以及下界和末地的 2D 维度尚未实现。其他模组如果同时修改世界生成、渲染、移动、碰撞或交互，可能与本模组冲突。

树木生成规则作用于新生成的区块。已有存档中的前景树木需要另行处理或由玩家拆除。此轮没有给旧区块执行批量树木搬迁。

## 3. 从旧格式到新格式

### 3.1 新清单

新版原生模组使用扁平 `.hmod` 包，包根目录包含 `horizon-mod.toml`、`Cargo.toml` 和 `src/`。资源可以放在 `assets/` 和 `data/`。图标为根目录的 `icon.png`。

`horizon-mod.toml` 提供 ID、版本、许可证和构建固定信息。`[native]` 提供 Minecraft 版本、Horizon 完整提交号及 Mixin 工具完整提交号。

原生 Mixin 声明采用 `[[native.mixin]]`，支持模块采用 `[[native.module]]`。启动器会生成内部的 `horizon-mixin.toml`。官方新规范已经移除输入 `horizon-native.json` 和手写工具清单的支持。

参考官方规范如下。

- https://github.com/horizon-rust/horizon/blob/main/docs/modding/src/start/first-mod.md
- https://github.com/horizon-rust/horizon/blob/main/docs/modding/src/reference/manifest.md
- https://github.com/horizon-rust/horizon/blob/main/docs/modding/src/concepts/native-route.md

### 3.2 Rust 项目结构

原来的 `api/` 与 `state/` 工作区已合并为一个模组 crate。共享模式选择状态保存在 `src/state.rs`。跨辅助模块使用的平面状态与常量位于 `src/plane.rs`。

模组入口依赖 `.horizon/sdk/api/crates/horizon-mod-api`。`.horizon/` 由官方工具获取并验证，属于本地缓存，不应提交或打进发布包。

启动器原生构建拒绝包级 `[workspace]`，因此根 `Cargo.toml` 采用普通单 crate 结构。注册表依赖使用官方 `native-lock` 生成的锁文件。开发阶段直接执行 Cargo 可能补入 SDK 的本地依赖记录，打包前需要重新生成官方原生锁文件。

### 3.3 编译检查的层次

三个检查各自覆盖不同内容。

| 检查 | 覆盖范围 |
| --- | --- |
| `cargo check` | 模组 SDK 入口及被入口引用的模块 |
| `cargo horizon-mod check <文件>` | 清单、路径、资源、包结构和声明合法性 |
| `horizon-launcher native-build` | 固定 SDK 下的辅助代码、函数替换库、编译与链接 |

本项目的大量代码由支持模块和 Mixin 辅助文件注入游戏，因此必须执行完整原生构建。函数出现在 SDK 索引中，仅证明存在可使用的目标，辅助代码的类型和可见性仍需要编译验证。

## 4. 文件地图

| 文件 | 责任 |
| --- | --- |
| `horizon-mod.toml` | 模组身份、固定版本、支持模块、原生声明 |
| `Cargo.toml` | SDK 路径和注册表依赖 |
| `src/lib.rs` | 初始化日志、创建世界按钮、标准 SDK 钩子 |
| `src/state.rs` | 按钮选择状态的原子读写 |
| `src/plane.rs` | 平面常量、作用域状态、背景交互状态 |
| `src/confirm.rs` | 创建或打开世界时的模式确认 |
| `src/streaming.rs` | 将主世界流式生成切换到平面生成器 |
| `src/terrain.rs` | 原版地形生成、切片、存档标记、区块整理 |
| `src/trees.rs` | 原版树特征过滤、背景表土和 2D 树布局 |
| `src/spawn.rs` | 平面世界的初始出生位置 |
| `src/side.rs` | 玩家和生物位置、视觉朝向、指针选取、高亮 |
| `src/pointer.rs` | 屏幕与世界之间的映射、2D 射线数学 |
| `src/physics.rs` | 物理计算期间的前景视图与背景恢复 |
| `src/upload.rs` | 上传前重建平面网格并应用背景与光照 |
| `src/background.rs` | 前景、背景分开网格化与合并 |
| `src/lighting.rs` | 2D 天空光、方块光和逐层遮蔽 |
| `src/canopy.rs` | 树冠遮住树干的视觉覆盖面 |
| `src/hooks/` | 每个原生声明的辅助函数 |
| `assets/planar_worlds/lang/` | 按钮文字的英文与简体中文翻译 |
| `README.md` | 英文项目说明 |
| `README.zh.md` | 中文项目说明 |

`target/` 中保留了旧阶段的编译工作区、测试夹具、签名包和本轮诊断。这些都是本机工作产物。重新克隆后应根据清单和 SDK 重建，不能把它们当成发布源代码的必要依赖。

## 5. 世界创建、持久化与生成

### 5.1 模式选择

创建世界按钮写入原子状态。确认创建世界时检查模式是否开启，成功写入存档目录标记后消费这次按钮选择。后续重新打开世界依据存档标记识别模式。

当前切片模式的标记文件名为 `planar-worlds-v2`。代码也识别更早的 `planar-worlds-v1` 标记，以保留旧存档的判定逻辑。标记保存在对应世界的 `saves/<世界目录>/` 内。

创建标记前检查 `level.dat` 与 `level.dat_old`，防止把既有世界误当作新世界初始化。创建失败时允许同一新世界重试并复用已有标记。

### 5.2 地形切片

`PlanarTerrain` 使用原版生成器获取完整 X/Y 地形变化，再清理切片之外的单元。切片包含地表、山体、洞穴、矿石、水、岩浆与基岩。

存储仍遵循普通区块布局。每个区块内部保留前景 Z 层，其他位置为空气。背景土壤和树木在切片整理后追加到 `Z=1`。清理过程也处理方块实体、方块名称、后处理坐标和生成实体，避免残留来自其他 Z 位置的数据。

### 5.3 原版树过滤

生成前递归检查配置特征和放置特征，把原版树、倒树等特征改成空操作。嵌套随机选择器中的内联特征也需要递归处理。

区块切片时再过滤原版树叶、原木、木头和红树根等状态，减少来自跨 Z 空间树木的悬空残片。这个过滤发生在生成阶段，玩家后续放置的木头不会经过该阶段。

### 5.4 2D 背景树

对每个前景 X 列寻找合适的土壤表面。表面为草方块、灰化土、菌丝体或苔藓等可种树土壤时，将地表及其下方两层复制到背景。

树木候选位置限制在树冠能完整放入当前区块的范围内。树干高度为四至五格，树冠两侧对称。树干与树叶全部写入 `plane::BACKGROUND`。

树的全部目标单元需要可替换，并且树冠不能超过世界最高位置。空间不足时尝试较矮的树或其他候选位置。

## 6. 旧注入的替代实现

此轮不以保留旧声明数量为目标。多个旧注入可以由一个标准钩子或上层调用处统一实现。最终声明数量应以交付包清单为准。

| 旧目标或功能 | 替代方式 |
| --- | --- |
| `gfx::world_projection` | 在游戏层相机矩阵中补偿透视投影，生成正交结果 |
| 水生生物的 `aquatic::rotations` | 在侧视绘制期间调整朝向及水生动画状态，绘制后恢复 |
| 多种实体的低层 `bounds` | 保留可替换的游戏层边界入口，并由统一移动平面和前景物理视图提供 2D 行为 |
| 玩家 `MotionWorld::move_self` | 包围本地玩家模拟入口，限制 Z 速度和位置，使用前景物理视图 |
| 服务端 `Hub::move_player` | 通过标准 `entity_move` 限制服务器接受的 Z 位移 |
| 生物 `do_move` | 通过统一实体移动钩子及实体阶段的平面约束处理 |
| `Islands::plan` | 使用会影响实体移动结果的标准钩子，遵循官方对相应生物执行路径的串行约束 |
| 两类低层背景碰撞查询 | 在物理阶段隐藏背景碰撞数据，阶段结束后恢复 |
| 原版树特征与树干底层函数 | 世界生成配置覆盖，加上最终切片中的树状态清理和手动 2D 树生成 |
| `Streaming::region` 与网格标记 | 在游戏接收完成网格的上传阶段处理 |
| `Mesher::mesh_reusing` | 上传前通过公开网格 API 分别重建前景、背景并着色 |
| 私有光照表读取 | 使用公开的方块透光度与发光强度进行 2D 传播 |
| 低层裂纹 `breaking::geometry` | 在游戏层调用处转换背景裂纹的渲染位置 |
| 全局 `World::set` | 在游戏层 `LevelAccess::set_raw` 检查写入平面 |
| 已移动的选取、输入和预测方法 | 按固定 SDK 的实际模块路径重绑，并调整缓存和参数类型 |
| 旧 `native_movement_input` 绑定 | 使用标准 `movement_input` SDK 事件 |
| 旧 `native_block_spread` 绑定 | 使用标准 `block_spread` SDK 事件 |

### 6.1 正交投影补偿

渲染器最终使用透视矩阵 P、相机效果矩阵 B 和视图矩阵 V 的乘积。模组在相机效果入口返回 `P.inverse() * O`，其中 O 为期望的正交矩阵。最终乘积成为 `O * V`。

该方法依赖渲染器的投影参数与辅助代码一致，包括 FOV、屏幕比例、近平面与远平面。更新 Horizon 后必须重新检查这些参数。投影回归测试覆盖横屏、竖屏、方形窗口，以及多个观察深度。

### 6.2 前景物理视图

`physics::run` 保存背景单元，临时从物理查询所见的区块中清除背景层，执行玩家或实体模拟，再把背景单元恢复。恢复阶段只覆盖背景位置，因此前景的合法修改会保留。

调用返回错误时仍然恢复背景。调用 panic 时先恢复背景，再重新抛出 panic。测试覆盖了错误路径和 panic 路径。

这种方法会增加区块复制、解包和重新打包工作。后续性能优化应优先测量这里，尤其是较大渲染距离和大量生物的场景。

### 6.3 上传前网格处理

上传阶段取得完成的网格列表，为平面世界读取当前区块与相邻列，分别生成前景和背景网格。背景网格再施加视觉深度偏移和暗色处理，与前景合并后重新整理上传。

这一阶段可以同时处理背景树的遮挡、剖面光照与背景深度。它会把一部分网格计算移到游戏线程，首次加载和批量修改时需要关注帧时间。

## 7. 光照与视觉规则

2D 光照沿 X/Y 邻接传播。前景的透光度决定天空光能否进入，方块发光强度提供火把等光源。进入实体岩层后逐层衰减，遮住无光处矿石的纹理。

光照使用公开的 `dampening` 和 `emission` 接口。相比旧实现对私有面遮蔽表的读取，半透明方块、台阶和其他非完整形状的遮蔽可能存在差异。后续应针对玻璃、树叶、台阶、楼梯和水下场景进行实机检查。

背景层使用剖面可见位置的光照结果，再施加暗色处理。地表背景树可获得天空光，深层背景墙会随所在剖面变暗。

树冠覆盖面属于渲染几何。树干数据仍保存在背景层，Ctrl 交互读取真实存储坐标。维护时需要同时理解存储坐标和视觉深度偏移。

## 8. 构建与发布操作

### 8.1 常规开发

在项目目录使用官方工具获取固定 SDK，再检查入口。

```powershell
cargo horizon-mod hooks camera
cargo check
```

SDK 目录需要访问官方发布资源。下载文件应按官方签名和 SDK 索引验证。不要为了绕过下载或缓存错误而填写伪造的签名、修改校验逻辑或放宽固定提交要求。

### 8.2 完整原生构建

```powershell
horizon-launcher native-lock . --update-lock
horizon-launcher native-build . --instance horizon --trust-native-code
```

运行构建前确认命令行中的启动器版本支持当前 `.hmod` 规范。`native-build` 只负责编译与选择构建，游戏窗口由人类启动。

### 8.3 打包与检查

```powershell
cargo horizon-mod build
cargo horizon-mod check target/horizon-mod/planar_worlds.hmod --json
```

发布文件采用包含版本号的名称，例如 `planar_worlds-0.3.7.hmod`。复制或重命名之后重新计算 SHA-256。签名必须对应最终上传的那份字节内容。

### 8.4 构建隔离

代理执行构建时，应使用独立的 `HORIZON_HOME`、`HORIZON_LAUNCHER_HOME`、`XDG_DATA_HOME` 和 `XDG_CONFIG_HOME`。启动器垃圾箱、剪贴板和桌面目录也应指向临时目录。设置 `HORIZON_LAUNCHER_NO_SELF_UPDATE=1` 和 `HORIZON_NO_AUDIO=1`。

本轮构建临时数据位于 `target/migration-0.3.7/`。该目录中的 `Run-Launcher.ps1` 记录了隔离环境。Windows 上启动器属于 GUI 子系统程序，脚本使用隐藏窗口并等待进程退出，确保收集到真实退出码。

Windows 编译器启动过长路径中的构建脚本时曾返回系统错误 2。隔离启动器缓存随后迁到 `D:/Horizon_Module/horizon/.local/p37`，缩短嵌套的 SDK、Mixin 工具和 Cargo 输出路径。这个目录属于本机缓存，已经位于 Horizon 的 Git 忽略范围内。

### 8.5 发布前验收

1. 核对模组 ID、版本、Horizon 提交和 Mixin 提交。
2. 运行 SDK 入口检查和针对性回归测试。
3. 跑通完整原生构建及链接。
4. 使用官方工具打包并检查最终文件。
5. 完成目标实例安装并核对文件哈希。
6. 由人类执行下节实机测试。
7. 对最终包签名，再发布 GitHub Release 或提交登记处。

## 9. 人类实机测试清单

### 9.1 创建与重新打开世界

- 创建新世界时显示 Planar Mode 按钮，文字能在 ON/OFF 之间切换。
- 开启后新世界生成地形横截面，日志出现初始化和模式选择记录。
- 保存退出后重新打开，同一世界继续使用平面模式。
- 创建普通世界后检查鼠标和摄像机模式正常切换。

### 9.2 摄像机与实体

- 改变窗口比例、缩放与分辨率，观察世界比例和鼠标格子是否对应。
- 移动鼠标、按 F5、受到伤害或进入水中时，侧视角保持稳定。
- A/D、跳跃、击退、游泳和飞行时，玩家始终停留在平面内。
- 普通生物、鱼、鱿鱼、掉落物、矿车、船和投射物保持 2D 行为。
- 玩家与生物左右切换时，画面不出现朝向中间的插值帧。

### 9.3 方块交互

- 从玩家眼睛到鼠标之间有墙时，选中第一个遮挡方块。
- 按住左键，确认裂纹、音效、破坏进度和掉落正常。
- 在五格范围内指向空气放置，检查超出范围和玩家占用位置的拒绝。
- Ctrl 切换背景目标，检查灰色高亮、背景树挖掘和背景墙放置。
- 背景方块的裂纹与其视觉位置重合。

### 9.4 树木与光照

- 在新生成的草地上观察背景土壤、连续树干和对称树冠。
- 玩家从树前走过时，树干与树叶不阻挡移动或形成落脚点。
- 草地斜坡、世界高度边界、区块边缘和水边没有悬空树冠。
- 地表树木明亮，背景树比前景略暗。
- 深处矿石在无光时被遮蔽，放火把后逐步可见。
- 测试玻璃、台阶、楼梯、树叶和水的遮光表现。

### 9.5 性能与存档

- 观察首次进入、快速跑图和批量放置/破坏时的卡顿。
- 使用较大渲染距离和较多生物检查物理视图复制开销。
- 重启后确认前景和背景方块、树木、物品栏和挖掘结果保存正确。
- 有其他模组或着色器时单独记录冲突组合和日志。

## 10. 故障定位

| 现象 | 优先检查 |
| --- | --- |
| 包在模组列表中不可识别 | 扩展名、根清单、包内层级、启动器版本 |
| 入口 Cargo 检查成功而原生构建失败 | `native.mixin` 目标、参数绑定、私有类型、函数替换日志 |
| 目标函数不存在 | 固定版本的 `functions.json` 和真实源文件模块路径 |
| `E_MIXIN_NOT_REPLACEABLE` | 标准 SDK 钩子或可替换的游戏层调用处 |
| `E_SELECTOR` / `E_MATCH_COUNT` | selector、ordinal、require 与函数体的实际匹配 |
| `E_INVALID_PATH` 指向官方源码中的 `../` | 区分旧目录发现流程和正式函数替换流程，保留完整日志 |
| 注册表锁文件被拒绝 | 使用官方 `native-lock`，检查 `[workspace]` 和路径依赖 |
| 签名与项目密钥不匹配 | 实际上传文件、签名文件、公钥三者是否为同一套 |
| 旧树仍在前景 | 是否处于已生成的旧区块 |
| 背景树全黑 | 2D 天空光结果、背景着色、网格上传是否执行 |
| 玩家撞到背景 | 物理视图入口是否执行、异常后是否恢复、实例是否使用新构建 |
| 鼠标选择位置偏移 | 窗口尺寸、GUI 缩放、相机中心、正交半高 |

日志应保留错误前后的上下文和完整编译命令。对外分享前检查本机路径和其他用户信息。

## 11. 签名与密钥保管

项目此前使用 Minisign。密钥材料位于本机 `C:/Users/36376/AppData/Local/PlanarMods/Signing/`。

私钥文件为 `planar_worlds.minisign.key`。受保护的口令记录为 `planar_worlds.password.dpapi`，依赖相应 Windows 用户的 DPAPI。迁移到其他账户或机器时，需要明确安排可恢复的密钥备份，不能把 DPAPI 文件本身当作跨机器可用的口令。

本文只记录保管位置。私钥内容、口令及 GitHub 凭据均不应写入仓库、README、日志或登记处表单。

任何包字节的变化都需要重新签名。`0.3.6` 的签名只对应当时校验过的文件，不能复用于 `0.3.7`。更新已登记项目的公钥需要遵循登记处的密钥变更流程。

## 12. 回滚与后续维护

回滚时保留世界存档，恢复先前使用的原生构建和对应模组版本。旧 `.hnmod` 需要支持它的启动器与构建流程。新版启动器使用 `.hmod`。

本轮的旧源码和生成工作区保留在 `target/` 中，适合本机对照。长期维护应以 Git 提交、版本标签及发布包为依据，并避免把官方游戏资源或编译缓存提交到 Git。

后续优先事项如下。

1. 人类完成 0.3.7 的完整实机测试并记录结果。
2. 测量背景物理视图和上传前重网格化的性能。
3. 增补半透明与非完整方块的剖面光照回归。
4. 讨论是否需要对旧区块的前景树提供显式迁移工具。
5. 在独立设计与测试后实现下界、末地及传送门。
6. 每次更新 Horizon SDK 时重跑完整原生构建和交互测试。

## 13. 开发归属与测试责任

原项目 README 的既有声明为代码由 ChatGPT 6.1 Sol 生成，人类负责架构设计和测试。本轮迁移由 Codex 代理执行源码检索、实现、终端检查与交付整理。

代理的编译和回归测试记录不能替代人类对实际游戏画面的验证。实机截图、体验结论和发布验收应由维护者确认并附到相应版本记录中。

## 附录 A. 原 29 处声明的逐项去向

以下 ID 对应迁移开始时被函数索引检查标出的 29 处声明。

| 原声明 ID | 本轮处理位置与方式 |
| --- | --- |
| `side-aquatic-pose` | `side.rs` 的 `ModelPose`，保存和恢复水生动画，绘制时校正侧向姿态 |
| `orthographic` | `hooks/side-no-bob.rs` 和 `hooks/side-lens.rs`，相机矩阵补偿与固定有效 FOV |
| `side-pick-unfiltered` | 绑定到 `game::client_targeting::Game::pick_unfiltered` |
| `side-default-outline` | 绑定到 `game::native_hooks::base::Game::native_block_highlight` |
| `side-mouse-buttons` | 绑定到 `game::play_input::Game::play_mouse`，调用现行鼠标映射入口 |
| `side-reticle` | 绑定到 `game::native_hooks::gui_render_post::Game::native_gui_render_post` |
| `side-input` | `Hooks::movement_input`，独立处理左右轴与面向 |
| `side-world-write` | 新声明 `planar-level-write`，在 `LevelAccess::set_raw` 检查当前交互层 |
| `side-player-bounds` | 玩家固定 Z，前景物理视图和本地模拟入口约束 |
| `side-player-move` | 新声明 `planar-player-physics`，包围 `tick_local_player` |
| `side-mob-bounds` | 新声明 `planar-mob-physics`，为实体阶段提供前景物理视图 |
| `side-server-move` | `Hooks::entity_move`，限制服务器物理位移的 Z 分量 |
| `side-spread` | `Hooks::block_spread`，阻止流体与火向其他 Z 层扩散 |
| `side-mob-move` | 统一实体移动钩子，加上实体阶段前后的中心位置约束 |
| `side-item-bounds` | 使用固定平面上的实体位置与前景物理视图 |
| `side-orb-bounds` | 使用固定平面上的实体位置与前景物理视图 |
| `side-stand-bounds` | 使用固定平面上的实体位置与前景碰撞约束 |
| `side-projectile-bounds` | 统一移动门限制 Z，实体阶段使用前景物理视图 |
| `side-fluid-boundary` | 绑定到 `game::level_access::<GameLevel as LevelAccess>::block_state` |
| `side-mob-serial` | 由会影响实体移动的隔离钩子参与官方的生物执行策略 |
| `planar-remote-use` | 绑定到 `game::remote::interaction_prediction::Game::remote_use_item_on`，适配 `Cell` 目标缓存 |
| `planar-tree-feature` | `trees.rs` 的生成覆盖和 `terrain.rs` 的树状态清理 |
| `planar-tree-trunk` | 由树特征过滤、树状态清理和手动背景树代替 |
| `background-crack` | 新声明 `planar-crack-origin`，修改游戏层裂纹调用的 origin 参数 |
| `background-predict-break` | 绑定到 `game::remote::break_prediction::Game::predict_break` |
| `background-collision` | `physics.rs`，物理阶段移除背景单元，结束后恢复 |
| `background-lookup-collision` | 同一前景物理视图覆盖相应查询 |
| `mark-slice-mesh` | `upload.rs` 直接使用当前世界与相邻区块数据 |
| `shade-slice-mesh` | 新声明 `planar-upload`，在上传前应用 2D 光照及背景网格处理 |

玩家与普通生物的部分低层 AABB 不再经过逐个方法的厚度覆写。新的实现通过固定平面、统一移动限制和前景物理视图提供 2D 碰撞行为。船、矿车、TNT 等仍有可替换的游戏层边界辅助逻辑。若后续需求包含向其他模组公开“所有实体的 Z 厚度都等于同一数值”的契约，需要单独设计和验证。

## 附录 B. 本轮本机目录

| 路径 | 用途 |
| --- | --- |
| `target/migration-0.3.7/launcher/` | 下载并校验过的官方启动器命令行程序 |
| `target/migration-0.3.7/sdk-mirror/` | 官方 SDK 分片和签名文件的本地副本 |
| `target/migration-0.3.7/scratch/` | 本轮隔离游戏目录与命令日志 |
| `D:/Horizon_Module/horizon/.local/p37/` | 缩短路径后的隔离启动器缓存、源码、Rust 和原生构建 |
| `target/migration-0.3.7/tests/` | 本轮的针对性测试夹具与输出 |
| `target/migration-0.3.7/legacy/` | 迁移前清单、入口与状态 crate 的本机备份 |
| `target/registry/` | 历史登记处上传材料，使用前必须核对版本与哈希 |
| `target/release-v0.3.6/` | 历史 0.3.6 Release 准备材料 |

这些目录用于复现本机工作过程。源码仓库的长期可构建性应通过官方固定版本和构建命令保证。

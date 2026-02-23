# MC 群系查找器 (mc-biome-finder)

一个跨平台的 Minecraft Java 版群系查找命令行工具，基于 [cubiomes](https://github.com/Cubitect/cubiomes) 群系生成库。

## 功能

- 在指定种子和版本下，搜索目标群系最密集的区域
- 支持 Minecraft 1.0 ~ 1.21 所有版本
- 支持所有主世界群系（含 1.18+ 新群系如 cherry_grove、pale_garden 等）
- 滑动窗口扫描 + 多线程并行搜索（rayon）
- 按群系密度和距原点距离排序输出

## 编译

需要 Rust 工具链和 C 编译器（MSVC / GCC / Clang）。

```bash
cargo build --release
```

编译产物在 `target/release/mc-biome-finder`（Windows 下为 `.exe`）。

编译时会自动从 `origin/` 目录编译 cubiomes C 源码并静态链接，生成的二进制文件完全独立，运行时无需任何外部依赖。

## 使用

```bash
mc-biome-finder --seed <种子> --version <版本> --biome <群系> [选项]
```

### 参数

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `--seed` | 世界种子（必填） | - |
| `--version` | MC 版本，如 1.18, 1.20, 1.21 | 1.21 |
| `--biome` | 目标群系名称（必填） | - |
| `--size` | 搜索窗口大小（区块边长） | 16 |
| `--origin-x` | 搜索原点 X 坐标 | 0 |
| `--origin-z` | 搜索原点 Z 坐标 | 0 |
| `--radius` | 搜索半径（方块） | 10000 |
| `--count` | 返回结果数量 | 10 |
| `--large-biomes` | 使用大型群系 | false |

### 示例

搜索蘑菇岛：
```bash
mc-biome-finder --seed 123456789 --version 1.18 --biome mushroom_fields --radius 10000
```

搜索樱花树林：
```bash
mc-biome-finder --seed 42 --version 1.20 --biome cherry_grove --size 8 --radius 5000
```

搜索苍白花园（1.21）：
```bash
mc-biome-finder --seed 999 --version 1.21 --biome pale_garden --size 8 --radius 5000
```

### 输出示例

```
MC 群系查找器
==============
种子: 123456789
版本: MC 1.18
目标群系: mushroom_fields
窗口大小: 16×16 区块
原点: (0, 0)
搜索半径: 10000 方块

搜索中...

找到 10 个结果（群系: mushroom_fields, 窗口: 16×16 区块）:

 #    | 坐标 (X, Z)          | 群系区块数      | 距原点距离
------------------------------------------------------------
 1    | ( -4496,   6640)   |      256   | 8019
 2    | ( -4880,   6384)   |      256   | 8036
 ...
```

## 支持的群系名称

常用群系：`plains`, `desert`, `forest`, `taiga`, `swamp`, `jungle`, `mushroom_fields`, `badlands`, `dark_forest`, `birch_forest`, `savanna`, `meadow`, `cherry_grove`, `pale_garden`, `deep_dark`, `mangrove_swamp`, `grove`, `snowy_slopes`, `jagged_peaks`, `frozen_peaks`, `stony_peaks` 等。

同时支持旧版名称别名，如 `ice_plains` → `snowy_tundra`，`mesa` → `badlands`。

## 项目结构

```
├── build.rs          # 编译 cubiomes C 代码
├── Cargo.toml        # Rust 项目配置
├── src/
│   ├── main.rs       # CLI 入口
│   ├── ffi.rs        # cubiomes FFI 绑定
│   ├── biome.rs      # 群系定义与名称解析
│   ├── generator.rs  # 安全的生成器封装
│   └── search.rs     # 多线程搜索算法
├── origin/           # cubiomes C 源码（编译时依赖）
└── docs/
    ├── requirements.md
    └── plan.md
```

## 路线图

- [x] Phase 1: CLI 工具
- [ ] Phase 2: WASM + Web 地图应用（类 Chunkbase）

## 致谢

- [cubiomes](https://github.com/Cubitect/cubiomes) — Minecraft 群系生成 C 库
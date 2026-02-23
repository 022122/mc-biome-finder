# MC 群系查找器 — 需求文档

## 项目概述

基于 cubiomes（C 语言 Minecraft 群系生成库）用 Rust 重写群系生成核心，构建一个跨平台的 Minecraft 群系查找工具。项目分两个阶段：命令行工具 和 Web 地图应用。

---

## 第一阶段：命令行工具（CLI）

### 目标

用 Rust 实现一个三端通用（Windows / macOS / Linux）的命令行工具，用户输入参数后，工具在指定范围内搜索符合条件的群系聚集区域，并返回结果。

### 输入参数

| 参数 | 说明 | 示例 |
|------|------|------|
| `--seed` | 世界种子（64位整数） | `123456789` |
| `--version` | Minecraft Java 版本 | `1.18`、`1.20`、`1.21` |
| `--biome` | 目标群系名称或 ID | `mushroom_fields`、`cherry_grove` |
| `--size` | 搜索窗口大小（区块数，边长） | `16`（即 16×16 区块的正方形窗口） |
| `--origin` | 搜索原点坐标（方块坐标） | `0,0` 或 `1000,-500` |
| `--radius` | 以原点为中心的搜索半径（方块） | `10000` |
| `--count` | 返回结果数量 | `10`（默认值） |

### 搜索逻辑

1. 以 `--origin` 为中心，`--radius` 为半径，确定搜索范围
2. 在搜索范围内，以滑动窗口（`--size` × `--size` 区块）扫描
3. 对每个窗口位置，统计窗口内目标群系占据的区块数量
4. 筛选出目标群系区块数最多的位置
5. 如果有多个符合条件的位置，按距原点的距离升序排列
6. 返回最近的 `--count` 个结果

### 输出格式

```
找到 10 个结果（群系: mushroom_fields, 窗口: 16×16 区块）:

 #  | 坐标 (X, Z)     | 群系区块数 | 距原点距离
----|-----------------|-----------|----------
 1  | (  320,  -480)  |    142    |    576
 2  | ( -160,   640)  |    138    |    660
 3  | ( 1024,   256)  |    135    |   1055
...
```

### 技术要求

- 语言：Rust
- 编译目标：x86_64-windows / x86_64-linux / aarch64-macos（三端通用）
- 参考实现：`origin/` 目录下的 cubiomes C 代码
- 需精确复现 Minecraft Java 版的群系生成算法（逐 bit 匹配）
- 支持多线程并行搜索以提升性能

---

## 第二阶段：Web 地图应用（WASM）

### 目标

将 Rust 群系生成核心编译为 WebAssembly，在浏览器中实现类似 [Chunkbase](https://www.chunkbase.com/apps/biome-finder) 的交互式地图界面。

### 功能要求

- **地图渲染**：在网页中以 2D 地图形式展示群系分布，支持缩放和拖拽
- **参数输入**：页面上提供种子、版本、群系、窗口大小、原点、半径等输入控件
- **搜索触发**：点击搜索后，在 WASM 中执行群系查找
- **结果展示**：
  - 地图上高亮标记符合条件的区域
  - 页面侧边栏以列表形式展示搜索结果（坐标、群系区块数、距离等）
  - 点击列表项可跳转到地图对应位置

### 技术要求

- Rust → WASM（wasm-pack / wasm-bindgen）
- 前端框架：轻量级即可（原生 JS / Canvas 或 Vue/React 均可）
- 地图渲染：Canvas 2D 或 WebGL
- 响应式布局，支持移动端浏览

---

## 开发路线

```
Phase 1: Rust CLI
  ├── 1.1 移植 RNG（rng.h → Rust）
  ├── 1.2 移植噪声生成（noise.c, biomenoise.c → Rust）
  ├── 1.3 移植层级生成（layers.c → Rust）
  ├── 1.4 移植生成器入口（generator.c → Rust）
  ├── 1.5 移植群系定义与查找表（biomes.c, tables/ → Rust）
  ├── 1.6 实现搜索算法（滑动窗口 + 排序）
  ├── 1.7 实现 CLI 参数解析与输出
  └── 1.8 单元测试（与 C 版本输出逐一对比验证）

Phase 2: WASM + Web
  ├── 2.1 将核心库编译为 WASM
  ├── 2.2 实现地图渲染（Canvas）
  ├── 2.3 实现搜索交互与结果展示
  └── 2.4 UI 优化与移动端适配
```

---

## 参考资料

- 原始 C 实现：`origin/` 目录（cubiomes 库）
- Cubiomes 项目：https://github.com/Cubitect/cubiomes
- Cubiomes Viewer：https://github.com/Cubitect/cubiomes-viewer
- Chunkbase：https://www.chunkbase.com/apps/biome-finder
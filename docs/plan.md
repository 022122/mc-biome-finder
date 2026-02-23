# MC 群系查找器 — 开发计划

> 此文档随开发进度持续更新

## 当前状态：✅ Phase 1 — Rust CLI 基本完成

---

## 策略

采用 **FFI 桥接 + 逐步替换** 的务实路线，而非一次性从零重写：

1. 先用 Rust 通过 FFI 调用 cubiomes C 库，快速搭建可用的 CLI 工具
2. CLI 功能验证通过后，再逐模块用纯 Rust 替换 C 代码
3. 这样可以尽早拿到可运行的产品，同时降低"精确复现"的风险

实际采用了 `cc` crate 在 build.rs 中编译 C 源码，跨平台兼容性好。

---

## Phase 1 详细步骤

### Step 1: 项目脚手架 ✅
- [x] 创建 Rust 项目结构（cargo init）
- [x] 配置 build.rs 编译 cubiomes C 代码（cc crate）
- [x] 验证 C 库能被 Rust 调用

### Step 2: FFI 绑定 ✅
- [x] 编写 cubiomes FFI 绑定（手写关键函数签名）
- [x] 封装安全的 Rust wrapper（BiomeGenerator）
- [x] 群系名称解析（parse_biome_name）

### Step 3: 搜索算法 ✅
- [x] 实现滑动窗口扫描逻辑
- [x] 实现结果排序（群系密度 + 距离）
- [x] 多线程并行搜索（rayon）

### Step 4: CLI 界面 ✅
- [x] 集成 clap 参数解析
- [x] 格式化输出结果表格
- [x] Release 构建测试

### Step 5: 测试与优化
- [x] 基本功能测试（mushroom_fields 搜索验证通过）
- [x] 性能测试（10000半径 ≈ 8.5秒，多线程）
- [ ] 更多群系测试
- [ ] 跨平台编译测试（Linux/macOS）
- [ ] 进一步性能优化（更大半径场景）

---

## Phase 2（后续）
- [ ] WASM 编译
- [ ] Canvas 地图渲染
- [ ] Web UI 搜索交互
- [ ] 侧边栏结果列表

---

## 变更记录

| 日期 | 变更 |
|------|------|
| 2026-02-23 | 初始计划创建，采用 FFI 桥接策略 |
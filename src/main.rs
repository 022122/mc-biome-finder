mod biome;
mod ffi;
mod generator;
mod search;

use biome::parse_biome_name;
use clap::Parser;
use generator::parse_mc_version;
use search::{SearchParams, search_biomes};

#[derive(Parser)]
#[command(name = "mc-biome-finder")]
#[command(about = "Minecraft Java Edition 群系查找工具")]
struct Cli {
    /// 世界种子
    #[arg(long)]
    seed: i64,

    /// Minecraft 版本 (如 1.18, 1.20, 1.21)
    #[arg(long, default_value = "1.21")]
    version: String,

    /// 目标群系名称 (如 mushroom_fields, cherry_grove)
    #[arg(long)]
    biome: String,

    /// 搜索窗口大小 (区块数, 正方形边长)
    #[arg(long, default_value = "16")]
    size: i32,

    /// 搜索原点 X 坐标 (方块)
    #[arg(long, default_value = "0")]
    origin_x: i32,

    /// 搜索原点 Z 坐标 (方块)
    #[arg(long, default_value = "0")]
    origin_z: i32,

    /// 搜索半径 (方块)
    #[arg(long, default_value = "10000")]
    radius: i32,

    /// 返回结果数量
    #[arg(long, default_value = "10")]
    count: usize,

    /// 使用大型群系
    #[arg(long, default_value = "false")]
    large_biomes: bool,
}

fn main() {
    let cli = Cli::parse();

    // Parse MC version
    let mc = match parse_mc_version(&cli.version) {
        Some(v) => v,
        None => {
            eprintln!("错误: 不支持的版本 '{}'", cli.version);
            eprintln!("支持的版本: 1.0 ~ 1.21（可输入如 1.18.2, 1.21.1, 1.21.11 等）");
            std::process::exit(1);
        }
    };

    // Parse biome name
    let target_biome = match parse_biome_name(&cli.biome) {
        Some(b) => b.as_c_int(),
        None => {
            eprintln!("错误: 未知群系 '{}'", cli.biome);
            eprintln!("示例: mushroom_fields, cherry_grove, plains, desert, jungle");
            std::process::exit(1);
        }
    };

    println!("MC 群系查找器");
    println!("==============");
    println!("种子: {}", cli.seed);
    println!("版本: MC {}", cli.version);
    println!("目标群系: {}", cli.biome);
    println!("窗口大小: {}×{} 区块", cli.size, cli.size);
    println!("原点: ({}, {})", cli.origin_x, cli.origin_z);
    println!("搜索半径: {} 方块", cli.radius);
    println!();
    println!("搜索中...");

    let params = SearchParams {
        seed: cli.seed as u64,
        mc,
        large_biomes: cli.large_biomes,
        target_biome,
        window_size: cli.size,
        origin_x: cli.origin_x,
        origin_z: cli.origin_z,
        radius: cli.radius,
        count: cli.count,
    };

    let results = search_biomes(&params, None);

    if results.is_empty() {
        println!("未找到符合条件的区域。");
        return;
    }

    println!(
        "\n找到 {} 个结果（群系: {}, 窗口: {}×{} 区块）:\n",
        results.len(),
        cli.biome,
        cli.size,
        cli.size
    );
    println!(
        " {:<4} | {:<22} | {:<10} | {}",
        "#", "方块坐标 (X, Z)", "群系区块数", "距原点(方块)"
    );
    println!("{}", "-".repeat(64));

    for (i, r) in results.iter().enumerate() {
        println!(
            " {:<4} | ({:>7}, {:>7})    | {:>8}   | {:.0}",
            i + 1,
            r.x,
            r.z,
            r.biome_chunks,
            r.distance
        );
    }
    println!();
    println!("提示: 坐标为方块坐标，可直接用 /tp @s X ~ Z 传送到对应位置");
}

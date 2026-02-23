use wasm_bindgen::prelude::*;
use serde::Serialize;
use std::os::raw::c_int;

// ============ FFI bindings (same as CLI) ============

#[repr(C, align(8))]
struct Generator {
    _data: [u8; 262144],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Range {
    scale: c_int,
    x: c_int,
    z: c_int,
    sx: c_int,
    sz: c_int,
    y: c_int,
    sy: c_int,
}

const MC_1_0: c_int = 3;
const MC_1_7: c_int = 14;
const MC_1_8: c_int = 15;
const MC_1_9: c_int = 16;
const MC_1_12: c_int = 19;
const MC_1_13: c_int = 20;
const MC_1_14: c_int = 21;
const MC_1_15: c_int = 22;
const MC_1_16: c_int = 24;
const MC_1_17: c_int = 25;
const MC_1_18: c_int = 26;
const MC_1_19: c_int = 28;
const MC_1_20: c_int = 29;
const MC_1_21: c_int = 32;
const DIM_OVERWORLD: c_int = 0;
const LARGE_BIOMES: u32 = 0x1;

extern "C" {
    fn setupGenerator(g: *mut Generator, mc: c_int, flags: u32);
    fn applySeed(g: *mut Generator, dim: c_int, seed: u64);
    fn genBiomes(g: *const Generator, cache: *mut c_int, r: Range) -> c_int;
}

// ============ Version parsing ============

fn parse_mc_version(version: &str) -> Option<c_int> {
    let parts: Vec<&str> = version.trim().split('.').collect();
    if parts.len() >= 2 && parts[0] == "1" {
        let minor: u32 = parts[1].parse().ok()?;
        match minor {
            0 => Some(MC_1_0),
            1..=6 | 7 => Some(MC_1_7),
            8 => Some(MC_1_8),
            9..=11 => Some(MC_1_9),
            12 => Some(MC_1_12),
            13 => Some(MC_1_13),
            14 => Some(MC_1_14),
            15 => Some(MC_1_15),
            16 => Some(MC_1_16),
            17 => Some(MC_1_17),
            18 => Some(MC_1_18),
            19 => Some(MC_1_19),
            20 => Some(MC_1_20),
            21.. => Some(MC_1_21),
        }
    } else {
        None
    }
}

// ============ Biome color map ============

/// Returns (r, g, b) for a biome ID, matching Minecraft's map colors.
fn biome_color(id: c_int) -> (u8, u8, u8) {
    match id {
        0 => (0, 0, 112),        // ocean
        1 => (141, 179, 96),     // plains
        2 => (250, 148, 24),     // desert
        3 => (96, 96, 96),       // mountains
        4 => (5, 102, 33),       // forest
        5 => (11, 102, 89),      // taiga
        6 => (7, 249, 178),      // swamp
        7 => (0, 0, 255),        // river
        8 => (87, 37, 38),       // nether_wastes
        9 => (128, 128, 255),    // the_end
        10 => (144, 144, 160),   // frozen_ocean
        11 => (160, 160, 255),   // frozen_river
        12 => (255, 255, 255),   // snowy_tundra
        13 => (160, 160, 160),   // snowy_mountains
        14 => (255, 0, 255),     // mushroom_fields
        15 => (160, 0, 255),     // mushroom_field_shore
        16 => (250, 222, 85),    // beach
        17 => (210, 95, 18),     // desert_hills
        18 => (34, 85, 28),      // wooded_hills
        19 => (22, 57, 51),      // taiga_hills
        20 => (114, 120, 154),   // mountain_edge
        21 => (83, 123, 9),      // jungle
        22 => (44, 66, 5),       // jungle_hills
        23 => (98, 139, 23),     // jungle_edge
        24 => (0, 0, 48),        // deep_ocean
        25 => (162, 162, 132),   // stone_shore
        26 => (250, 240, 192),   // snowy_beach
        27 => (48, 116, 68),     // birch_forest
        28 => (31, 95, 50),      // birch_forest_hills
        29 => (64, 81, 26),      // dark_forest
        30 => (49, 85, 74),      // snowy_taiga
        31 => (36, 63, 54),      // snowy_taiga_hills
        32 => (89, 102, 81),     // giant_tree_taiga
        33 => (69, 79, 62),      // giant_tree_taiga_hills
        34 => (80, 112, 80),     // wooded_mountains
        35 => (189, 178, 95),    // savanna
        36 => (167, 157, 100),   // savanna_plateau
        37 => (217, 69, 21),     // badlands
        38 => (176, 151, 101),   // wooded_badlands_plateau
        39 => (202, 140, 101),   // badlands_plateau
        44 => (0, 0, 172),       // warm_ocean
        45 => (0, 0, 144),       // lukewarm_ocean
        46 => (32, 32, 112),     // cold_ocean
        47 => (0, 0, 80),        // deep_warm_ocean
        48 => (0, 0, 64),        // deep_lukewarm_ocean
        49 => (32, 32, 56),      // deep_cold_ocean
        50 => (64, 64, 144),     // deep_frozen_ocean
        127 => (0, 0, 0),        // the_void
        129 => (181, 219, 136),  // sunflower_plains
        130 => (255, 188, 64),   // desert_lakes
        131 => (136, 136, 136),  // gravelly_mountains
        132 => (45, 142, 73),    // flower_forest
        133 => (51, 142, 129),   // taiga_mountains
        134 => (47, 255, 218),   // swamp_hills
        140 => (180, 220, 220),  // ice_spikes
        149 => (123, 163, 49),   // modified_jungle
        151 => (138, 179, 63),   // modified_jungle_edge
        155 => (88, 156, 108),   // tall_birch_forest
        156 => (71, 135, 90),    // tall_birch_hills
        157 => (104, 121, 66),   // dark_forest_hills
        158 => (89, 125, 114),   // snowy_taiga_mountains
        160 => (129, 142, 121),  // giant_spruce_taiga
        161 => (109, 119, 102),  // giant_spruce_taiga_hills
        162 => (120, 152, 120),  // modified_gravelly_mountains
        163 => (229, 218, 135),  // shattered_savanna
        164 => (207, 197, 140),  // shattered_savanna_plateau
        165 => (255, 109, 61),   // eroded_badlands
        166 => (216, 191, 141),  // modified_wooded_badlands
        167 => (242, 180, 141),  // modified_badlands_plateau
        168 => (118, 142, 20),   // bamboo_jungle
        169 => (59, 71, 10),     // bamboo_jungle_hills
        170 => (84, 48, 36),     // soul_sand_valley
        171 => (189, 50, 49),    // crimson_forest
        172 => (73, 144, 123),   // warped_forest
        173 => (80, 80, 97),     // basalt_deltas
        174 => (136, 112, 80),   // dripstone_caves
        175 => (56, 156, 56),    // lush_caves
        177 => (130, 197, 86),   // meadow
        178 => (120, 160, 120),  // grove
        179 => (200, 220, 230),  // snowy_slopes
        180 => (170, 170, 170),  // jagged_peaks
        181 => (210, 230, 240),  // frozen_peaks
        182 => (140, 140, 130),  // stony_peaks
        183 => (10, 10, 30),     // deep_dark
        184 => (40, 100, 40),    // mangrove_swamp
        185 => (255, 183, 197),  // cherry_grove
        186 => (72, 94, 72),     // pale_garden
        _ => (100, 100, 100),    // unknown
    }
}

// ============ WASM exports ============

#[derive(Serialize)]
pub struct BiomeSearchResult {
    pub x: i32,
    pub z: i32,
    pub biome_chunks: u32,
    pub distance: f64,
}

/// Generate a biome color map for a given area.
/// Returns a flat RGBA array (width * height * 4 bytes).
#[wasm_bindgen]
pub fn generate_biome_map(
    seed: i64,
    version: &str,
    large_biomes: bool,
    center_x: i32,
    center_z: i32,
    width: i32,
    height: i32,
    scale: i32,
) -> Vec<u8> {
    let mc = parse_mc_version(version).unwrap_or(MC_1_21);
    let flags = if large_biomes { LARGE_BIOMES } else { 0 };

    let mut gen: Box<Generator> = Box::new(unsafe { std::mem::zeroed() });
    unsafe {
        setupGenerator(&mut *gen, mc, flags);
        applySeed(&mut *gen, DIM_OVERWORLD, seed as u64);
    }

    // biome coords: divide block coords by 4, then by scale factor
    let biome_scale = if scale <= 1 { 1 } else { scale / 4 };
    let actual_scale = biome_scale.max(1) * 4; // back to block scale for range

    let bx = center_x / actual_scale - width / 2;
    let bz = center_z / actual_scale - height / 2;

    let r = Range {
        scale: actual_scale,
        x: bx,
        z: bz,
        sx: width,
        sz: height,
        y: 15,
        sy: 1,
    };

    let len = (width * height) as usize;
    let mut biomes = vec![0i32; len];
    unsafe {
        genBiomes(&*gen, biomes.as_mut_ptr(), r);
    }

    // Convert to RGBA
    let mut rgba = vec![0u8; len * 4];
    for i in 0..len {
        let (r, g, b) = biome_color(biomes[i]);
        rgba[i * 4] = r;
        rgba[i * 4 + 1] = g;
        rgba[i * 4 + 2] = b;
        rgba[i * 4 + 3] = 255;
    }
    rgba
}

/// Search for biome-dense areas. Returns JSON array of results.
#[wasm_bindgen]
pub fn search_biome(
    seed: i64,
    version: &str,
    large_biomes: bool,
    biome_id: i32,
    window_size: i32,
    origin_x: i32,
    origin_z: i32,
    radius: i32,
    count: usize,
) -> JsValue {
    let mc = parse_mc_version(version).unwrap_or(MC_1_21);
    let flags = if large_biomes { LARGE_BIOMES } else { 0 };

    let window_chunks = window_size;
    let step_chunks = (window_chunks / 2).max(1);
    let radius_biome = radius / 4;
    let origin_bx = origin_x / 4;
    let origin_bz = origin_z / 4;
    let window_biome = window_chunks * 4;
    let step_biome = step_chunks * 4;

    let scan_min_bx = origin_bx - radius_biome;
    let scan_max_bx = origin_bx + radius_biome - window_biome;
    let scan_min_bz = origin_bz - radius_biome;
    let scan_max_bz = origin_bz + radius_biome - window_biome;

    let mut gen: Box<Generator> = Box::new(unsafe { std::mem::zeroed() });
    unsafe {
        setupGenerator(&mut *gen, mc, flags);
        applySeed(&mut *gen, DIM_OVERWORLD, seed as u64);
    }

    let mut results: Vec<BiomeSearchResult> = Vec::new();

    let mut bz = scan_min_bz;
    while bz <= scan_max_bz {
        let strip_sx = scan_max_bx - scan_min_bx + window_biome;
        let strip_sz = window_biome;

        if strip_sx > 0 && strip_sz > 0 {
            let r = Range {
                scale: 4,
                x: scan_min_bx,
                z: bz,
                sx: strip_sx,
                sz: strip_sz,
                y: 15,
                sy: 1,
            };
            let len = (strip_sx * strip_sz) as usize;
            let mut biomes = vec![0i32; len];
            unsafe {
                genBiomes(&*gen, biomes.as_mut_ptr(), r);
            }

            let mut bx = scan_min_bx;
            while bx <= scan_max_bx {
                let mut biome_count = 0u32;
                for wz in 0..window_biome {
                    for wx in 0..window_biome {
                        let idx = (wz * strip_sx + (bx - scan_min_bx) + wx) as usize;
                        if idx < biomes.len() && biomes[idx] == biome_id {
                            biome_count += 1;
                        }
                    }
                }
                if biome_count > 0 {
                    let cx = (bx + window_biome / 2) * 4;
                    let cz = (bz + window_biome / 2) * 4;
                    let dx = (cx - origin_x) as f64;
                    let dz = (cz - origin_z) as f64;
                    let chunks = biome_count / 16;
                    if chunks > 0 {
                        results.push(BiomeSearchResult {
                            x: cx,
                            z: cz,
                            biome_chunks: chunks,
                            distance: (dx * dx + dz * dz).sqrt(),
                        });
                    }
                }
                bx += step_biome;
            }
        }
        bz += step_biome;
    }

    results.sort_by(|a, b| {
        b.biome_chunks
            .cmp(&a.biome_chunks)
            .then_with(|| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal))
    });

    // NMS dedup: skip results whose center is within window_blocks of an already-selected result
    let window_blocks = (window_size * 16) as f64;
    let mut selected: Vec<BiomeSearchResult> = Vec::with_capacity(count);
    for r in results {
        let dominated = selected.iter().any(|s| {
            let dx = (r.x - s.x) as f64;
            let dz = (r.z - s.z) as f64;
            (dx * dx + dz * dz).sqrt() < window_blocks
        });
        if !dominated {
            selected.push(r);
            if selected.len() >= count {
                break;
            }
        }
    }

    serde_wasm_bindgen::to_value(&selected).unwrap_or(JsValue::NULL)
}

/// Get biome name from ID
#[wasm_bindgen]
pub fn get_biome_name(id: i32) -> String {
    match id {
        0 => "ocean", 1 => "plains", 2 => "desert", 3 => "mountains",
        4 => "forest", 5 => "taiga", 6 => "swamp", 7 => "river",
        14 => "mushroom_fields", 21 => "jungle", 24 => "deep_ocean",
        27 => "birch_forest", 29 => "dark_forest", 35 => "savanna",
        37 => "badlands", 44 => "warm_ocean", 132 => "flower_forest",
        168 => "bamboo_jungle", 177 => "meadow", 178 => "grove",
        179 => "snowy_slopes", 180 => "jagged_peaks", 182 => "stony_peaks",
        183 => "deep_dark", 184 => "mangrove_swamp", 185 => "cherry_grove",
        186 => "pale_garden",
        _ => "unknown",
    }.to_string()
}
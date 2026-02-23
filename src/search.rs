//! Biome search algorithm: sliding window scan within a radius.
//! Uses rayon for parallel strip processing.

use crate::generator::BiomeGenerator;
use rayon::prelude::*;
use std::os::raw::c_int;
use std::sync::Mutex;

/// A search result: a window position with biome density info.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Block coordinate X of the window center
    pub x: i32,
    /// Block coordinate Z of the window center
    pub z: i32,
    /// Number of chunks in the window that contain the target biome
    pub biome_chunks: u32,
    /// Distance from origin (in blocks)
    pub distance: f64,
}

/// Search parameters
pub struct SearchParams {
    pub seed: u64,
    pub mc: c_int,
    pub large_biomes: bool,
    pub target_biome: c_int,
    /// Window size in chunks (e.g. 16 means 16x16 chunks)
    pub window_size: i32,
    /// Origin block coordinates
    pub origin_x: i32,
    pub origin_z: i32,
    /// Search radius in blocks from origin
    pub radius: i32,
    /// Max number of results to return
    pub count: usize,
}

/// Process a single strip (one Z row of the scan grid).
fn process_strip(
    bz: i32,
    params: &SearchParams,
    scan_min_bx: i32,
    scan_max_bx: i32,
    window_biome: i32,
    step_biome: i32,
) -> Vec<SearchResult> {
    let mut gen = BiomeGenerator::new(params.mc, params.large_biomes);
    gen.apply_seed(params.seed);

    let strip_sx = scan_max_bx - scan_min_bx + window_biome;
    let strip_sz = window_biome;

    if strip_sx <= 0 || strip_sz <= 0 {
        return Vec::new();
    }

    let biomes = gen.gen_biomes_area(scan_min_bx, bz, strip_sx, strip_sz);
    let mut results = Vec::new();

    let mut bx = scan_min_bx;
    while bx <= scan_max_bx {
        let mut count = 0u32;
        for wz in 0..window_biome {
            for wx in 0..window_biome {
                let idx = (wz * strip_sx + (bx - scan_min_bx) + wx) as usize;
                if idx < biomes.len() && biomes[idx] == params.target_biome {
                    count += 1;
                }
            }
        }

        if count > 0 {
            let center_x = (bx + window_biome / 2) * 4;
            let center_z = (bz + window_biome / 2) * 4;
            let dx = (center_x - params.origin_x) as f64;
            let dz = (center_z - params.origin_z) as f64;
            let distance = (dx * dx + dz * dz).sqrt();

            let biome_chunks = count / 16;
            if biome_chunks > 0 {
                results.push(SearchResult {
                    x: center_x,
                    z: center_z,
                    biome_chunks,
                    distance,
                });
            }
        }

        bx += step_biome;
    }
    results
}

/// Run the biome search with parallel strip processing.
pub fn search_biomes(params: &SearchParams, progress: Option<&Mutex<(u32, u32)>>) -> Vec<SearchResult> {
    let window_chunks = params.window_size;
    let step_chunks = (window_chunks / 2).max(1);

    let radius_biome = params.radius / 4;
    let origin_bx = params.origin_x / 4;
    let origin_bz = params.origin_z / 4;

    let window_biome = window_chunks * 4;
    let step_biome = step_chunks * 4;

    let scan_min_bx = origin_bx - radius_biome;
    let scan_max_bx = origin_bx + radius_biome - window_biome;
    let scan_min_bz = origin_bz - radius_biome;
    let scan_max_bz = origin_bz + radius_biome - window_biome;

    // Collect all Z positions to scan
    let mut z_positions = Vec::new();
    let mut bz = scan_min_bz;
    while bz <= scan_max_bz {
        z_positions.push(bz);
        bz += step_biome;
    }

    let total = z_positions.len() as u32;
    if let Some(p) = progress {
        let mut lock = p.lock().unwrap();
        lock.1 = total;
    }

    // Parallel scan using rayon
    let all_results: Vec<Vec<SearchResult>> = z_positions
        .par_iter()
        .map(|&bz| {
            let res = process_strip(bz, params, scan_min_bx, scan_max_bx, window_biome, step_biome);
            if let Some(p) = progress {
                let mut lock = p.lock().unwrap();
                lock.0 += 1;
            }
            res
        })
        .collect();

    let mut results: Vec<SearchResult> = all_results.into_iter().flatten().collect();

    // Sort: primary by biome_chunks descending, secondary by distance ascending
    results.sort_by(|a, b| {
        b.biome_chunks
            .cmp(&a.biome_chunks)
            .then_with(|| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal))
    });

    // NMS dedup: skip results whose center is within window_blocks of an already-selected result
    let window_blocks = (params.window_size * 16) as f64;
    let mut selected: Vec<SearchResult> = Vec::with_capacity(params.count);
    for r in results {
        let dominated = selected.iter().any(|s| {
            let dx = (r.x - s.x) as f64;
            let dz = (r.z - s.z) as f64;
            (dx * dx + dz * dz).sqrt() < window_blocks
        });
        if !dominated {
            selected.push(r);
            if selected.len() >= params.count {
                break;
            }
        }
    }
    selected
}
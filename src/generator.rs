//! Safe Rust wrapper around the cubiomes Generator.

use crate::ffi;
use std::os::raw::c_int;

/// Parse a version string like "1.18" or "1.20" into the cubiomes MC constant.
pub fn parse_mc_version(version: &str) -> Option<c_int> {
    match version {
        "1.0" => Some(ffi::MC_1_0),
        "1.7" => Some(ffi::MC_1_7),
        "1.8" => Some(ffi::MC_1_8),
        "1.9" => Some(ffi::MC_1_9),
        "1.12" => Some(ffi::MC_1_12),
        "1.13" => Some(ffi::MC_1_13),
        "1.14" => Some(ffi::MC_1_14),
        "1.15" => Some(ffi::MC_1_15),
        "1.16" => Some(ffi::MC_1_16),
        "1.17" => Some(ffi::MC_1_17),
        "1.18" => Some(ffi::MC_1_18),
        "1.19" => Some(ffi::MC_1_19),
        "1.20" => Some(ffi::MC_1_20),
        "1.21" => Some(ffi::MC_1_21),
        _ => None,
    }
}

/// Safe wrapper around the cubiomes Generator.
pub struct BiomeGenerator {
    inner: Box<ffi::Generator>,
    mc: c_int,
}

// The C Generator struct is used single-threaded per instance.
// We ensure safety by not sharing across threads without cloning.
unsafe impl Send for BiomeGenerator {}

impl BiomeGenerator {
    /// Create a new generator for the given MC version.
    pub fn new(mc: c_int, large_biomes: bool) -> Self {
        let mut inner = Box::new(unsafe { std::mem::zeroed::<ffi::Generator>() });
        let flags = if large_biomes { ffi::LARGE_BIOMES } else { 0 };
        unsafe {
            ffi::setupGenerator(&mut *inner, mc, flags);
        }
        BiomeGenerator { inner, mc }
    }

    /// Apply a seed for the overworld dimension.
    pub fn apply_seed(&mut self, seed: u64) {
        unsafe {
            ffi::applySeed(&mut *self.inner, ffi::DIM_OVERWORLD, seed);
        }
    }

    /// Get the biome at a single block position (scale=4 for biome coords).
    pub fn get_biome_at(&self, x: i32, y: i32, z: i32) -> c_int {
        unsafe { ffi::getBiomeAt(&*self.inner, 4, x, y, z) }
    }

    /// Generate biomes for a 2D area at scale 1:4 (biome coordinates).
    /// Returns a Vec of biome IDs indexed as [z * sx + x].
    pub fn gen_biomes_area(&self, bx: i32, bz: i32, sx: i32, sz: i32) -> Vec<c_int> {
        let r = ffi::Range {
            scale: 4,
            x: bx,
            z: bz,
            sx,
            sz,
            y: 15, // near sea level in biome coords (y=60 blocks / 4)
            sy: 1,
        };
        let len = (sx * sz) as usize;
        let mut cache = vec![0i32; len];
        unsafe {
            ffi::genBiomes(&*self.inner, cache.as_mut_ptr(), r);
        }
        cache
    }

    pub fn mc_version(&self) -> c_int {
        self.mc
    }
}
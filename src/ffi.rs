//! Raw FFI bindings to cubiomes C library.
//! We treat Generator as an opaque blob since its internal layout is complex (unions).

use std::os::raw::c_int;

/// Opaque Generator type - sized generously to hold the C struct.
/// The actual size depends on the platform, but 262144 bytes is more than enough.
#[repr(C, align(8))]
pub struct Generator {
    _data: [u8; 262144],
}

/// Range struct matching the C definition in biomenoise.h
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub scale: c_int,
    pub x: c_int,
    pub z: c_int,
    pub sx: c_int,
    pub sz: c_int,
    pub y: c_int,
    pub sy: c_int,
}

// MC version constants (from biomes.h enum MCVersion)
pub const MC_1_0: c_int = 3;
pub const MC_1_7: c_int = 14;
pub const MC_1_8: c_int = 15;
pub const MC_1_9: c_int = 16;
pub const MC_1_12: c_int = 19;
pub const MC_1_13: c_int = 20;
pub const MC_1_14: c_int = 21;
pub const MC_1_15: c_int = 22;
pub const MC_1_16: c_int = 24;
pub const MC_1_17: c_int = 25;
pub const MC_1_18: c_int = 26;
pub const MC_1_19: c_int = 28;
pub const MC_1_20: c_int = 29;
pub const MC_1_21: c_int = 32;

// Dimension constants
pub const DIM_OVERWORLD: c_int = 0;
#[allow(dead_code)]
pub const DIM_NETHER: c_int = -1;
#[allow(dead_code)]
pub const DIM_END: c_int = 1;

// Generator flags
pub const LARGE_BIOMES: u32 = 0x1;

extern "C" {
    pub fn setupGenerator(g: *mut Generator, mc: c_int, flags: u32);
    pub fn applySeed(g: *mut Generator, dim: c_int, seed: u64);
    pub fn getBiomeAt(g: *const Generator, scale: c_int, x: c_int, y: c_int, z: c_int) -> c_int;
    pub fn genBiomes(g: *const Generator, cache: *mut c_int, r: Range) -> c_int;
    pub fn allocCache(g: *const Generator, r: Range) -> *mut c_int;
}
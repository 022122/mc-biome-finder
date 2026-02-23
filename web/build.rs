fn main() {
    let mut build = cc::Build::new();

    // Core biome generation files only (no finders/quadbase/util which need pthread/windows)
    build
        .files(&[
            "../origin/noise.c",
            "../origin/biomes.c",
            "../origin/layers.c",
            "../origin/biomenoise.c",
            "../origin/generator.c",
            "wasm-stubs/wasm_libc.c",
        ])
        .include("../origin")
        .include("wasm-stubs")
        .flag("-nostdinc")
        .flag("-isystem")
        .flag("wasm-stubs")
        .flag_if_supported("-fwrapv")
        .flag_if_supported("-Wno-builtin-requires-header")
        .flag_if_supported("-Wno-incompatible-library-redeclaration")
        .opt_level(3);

    build.compile("cubiomes");

    println!("cargo:rerun-if-changed=../origin/");
    println!("cargo:rerun-if-changed=wasm-stubs/");
}
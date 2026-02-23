fn main() {
    let mut build = cc::Build::new();

    build
        .files(&[
            "origin/noise.c",
            "origin/biomes.c",
            "origin/layers.c",
            "origin/biomenoise.c",
            "origin/generator.c",
            "origin/finders.c",
            "origin/util.c",
            "origin/quadbase.c",
        ])
        .include("origin")
        .flag_if_supported("-fwrapv")
        .flag_if_supported("-Wall")
        .opt_level(3);

    // Windows specific
    if cfg!(target_os = "windows") {
        build.define("_WIN32", None);
    }

    build.compile("cubiomes");

    println!("cargo:rerun-if-changed=origin/");
}
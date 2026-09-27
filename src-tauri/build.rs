fn main() {
    cc::Build::new()
        .file("engine/core.c")
        .compile("fractal_engine");

    println!("cargo:rerun-if-changed=engine/core.c");
    println!("cargo:rerun-if-changed=engine/core.h");

    tauri_build::build();
}
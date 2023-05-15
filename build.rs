use std::process::Command;

fn main() {
    // This tells Cargo to re-run the build script if the worker code changes.
    println!("cargo:rerun-if-changed=audio_worker/src/");
    println!("cargo:rerun-if-changed=static/rust_audio_processor.js");
    println!("cargo:rerun-if-changed=static/text_decoder.js");
    println!("cargo:rerun-if-changed=audio_worker/Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:warning=Building worker code");

    // Build the worker's Rust code as WebAssembly
    let status = Command::new("cargo")
        .args(&[
            "build",
            "--target",
            "wasm32-unknown-unknown",
            "--release",
            "--manifest-path",
            "audio_worker/Cargo.toml",
        ])
        .status()
        .expect("Failed to build worker's Rust code");

    if !status.success() {
        panic!("Failed to build worker's Rust code");
    }

    // Run wasm-bindgen to generate the JavaScript bindings
    let status = Command::new("wasm-bindgen")
        .args(&[
            "--out-dir",
            "static/worker",
            "--target",
            "web",
            "--no-typescript",
            "audio_worker/target/wasm32-unknown-unknown/release/audio_worker.wasm",
        ])
        .status()
        .expect("Failed to run wasm-bindgen");

    if !status.success() {
        panic!("Failed to run wasm-bindgen");
    }

    // Bundle the worker JavaScript files using esbuild
    let status = Command::new(get_esbuild())
        .args(&[
            "--bundle",
            "static/rust_audio_processor.js",
            "--outfile=static/worker/bundled_rust_audio_processor.js",
            "--format=iife",
        ])
        .status()
        .expect("Failed to bundle worker JavaScript files");

    if !status.success() {
        panic!("Failed to bundle worker JavaScript files");
    }
}

fn get_esbuild() -> &'static str {
    if cfg!(target_os = "windows") {
        return "esbuild.cmd";
    }

    return "esbuild";
}

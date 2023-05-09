use std::process::Command;

fn main() {
    // This tells Cargo to re-run the build script if the worker code changes.
    println!("cargo:rerun-if-changed=audio_worker/src/");
    println!("cargo:rerun-if-changed=static/worker.js");

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
    let status = Command::new("esbuild.cmd")
        .args(&[
            "--bundle",
            "static/worker.js",
            "--outfile=static/bundled_worker.js",
            "--format=iife",
        ])
        .status()
        .expect("Failed to bundle worker JavaScript files");

    if !status.success() {
        panic!("Failed to bundle worker JavaScript files");
    }
}

use std::env;

fn main() {
    let mut build = cc::Build::new();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if os == "windows" {
        build.file("src/windows.c");
    } else if arch == "s390x" {
        build.file("src/unix/s390x.S");
    } else {
        // assume that this is included via inline assembly in the crate itself,
        // and the crate will otherwise have a `compile_error!` for unsupported
        // platforms.
        return;
    }
    build.define(&format!("CFG_TARGET_OS_{}", os), None);
    build.define(&format!("CFG_TARGET_ARCH_{}", arch), None);
    build.compile("wasmtime-fiber");
}

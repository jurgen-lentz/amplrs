use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let include_path = env::var("AMPL_INCLUDE")
        .expect("Set AMPL_INCLUDE to AMPL headers path");

    let lib_path = env::var("AMPL_LIB")
        .expect("Set AMPL_LIB to AMPL library path");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_path))
        .allowlist_function("AMPL_.*")
        .allowlist_type("AMPL_.*")
        .allowlist_var("AMPL_.*")
        .generate()
        .expect("bindgen failed");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=ampl");
}

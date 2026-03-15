fn main() {
    cxx_build::bridge("src/lib.rs").compile("app_bridge_cxx");

    println!("cargo:rerun-if-changed=src/lib.rs");
}

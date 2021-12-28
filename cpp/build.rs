fn main() {
    cxx_build::bridge("src/lib.rs").compile("vodozemac");
    println!("cargo:rerun-if-changed=src/lib.rs");
}

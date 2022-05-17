use cxx_build::CFG;

fn main() {
    CFG.include_prefix = "vodozemac";
    cxx_build::bridge("src/lib.rs").compile("vodozemac");
}

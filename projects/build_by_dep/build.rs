fn main() {
    let grammars = std::path::Path::new("grammars/").canonicalize().unwrap();
    let builder = yggdrasil_shared::codegen::RustCodegen::default();
    builder.generate(include_str!("grammars/json5.ygg"), "src/json5").unwrap();
    println!("cargo:rerun-if-changed={}", grammars.display());
}

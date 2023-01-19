use std::path::Path;

use yggdrasil_shared::codegen::RustCodegen;

fn main() {
    let grammars = Path::new("grammars/").canonicalize().unwrap();
    let builder = RustCodegen::default();
    builder.generate(include_str!("grammars/json5.ygg"), "src/json5").unwrap();
    println!("cargo:rerun-if-changed={}", grammars.display());
}

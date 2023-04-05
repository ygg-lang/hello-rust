#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

use crate::json5::{Json5Parser, Json5Rule, ValueNode};
use yggdrasil_rt::TokenTree;
pub use yggdrasil_rt::YggdrasilParser;

pub mod json5;

impl ValueNode {
    pub fn from_cst(tree: TokenTree<Json5Rule>) -> Self {
        todo!()
    }
}

#[test]
fn test_array() {
    let cst = Json5Parser::parse_cst("[1, null, ]", Json5Rule::Value).unwrap();
    let out = ValueNode::from_cst(cst);
    println!("{:?}", out)
}

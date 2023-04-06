#![allow(dead_code, unused_imports, non_camel_case_types)]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
#![allow(clippy::unnecessary_cast)]
#![doc = include_str!("readme.md")]

mod parse_cst;
mod parse_ast;

use std::{borrow::Cow, ops::Range, sync::OnceLock};
use yggdrasil_rt::*;

type Input<'i> = Box<State<'i, Json5Rule>>;
type Output<'i> = Result<Box<State<'i, Json5Rule>>, Box<State<'i, Json5Rule>>>;

#[doc = include_str!("railway.min.svg")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Json5Parser {}

impl YggdrasilParser for Json5Parser {
    type Rule = Json5Rule;
    fn parse_cst(input: &str, rule: Self::Rule) -> OutputResult<Json5Rule> {
        self::parse_cst::parse_cst(input, rule)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Json5Rule {
    Value,
    Object,
    ObjectPair,
    ObjectKey,
    Array,
    String,
    StringRaw,
    StringText,
    StringEscape,
    Number,
    Integer,
    Identifier,
    Boolean,
    Null,
    WhiteSpace,
    /// Label for text literal
    IgnoreText,
    /// Label for regex literal
    IgnoreRegex,
}

impl YggdrasilRule for Json5Rule {
    fn is_ignore(&self) -> bool {
        matches!(self, Self::IgnoreText | Self::IgnoreRegex | Self::WhiteSpace)
    }

    fn get_style(&self) -> &'static str {
        match self {
            Self::Value => "",
            Self::Object => "",
            Self::ObjectPair => "",
            Self::ObjectKey => "",
            Self::Array => "",
            Self::String => "",
            Self::StringRaw => "",
            Self::StringText => "",
            Self::StringEscape => "",
            Self::Number => "",
            Self::Integer => "",
            Self::Identifier => "",
            Self::Boolean => "",
            Self::Null => "",
            Self::WhiteSpace => "",
            _ => "",
        }
    }
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValueNode {
    Array(ArrayNode),
    Boolean(BooleanNode),
    Null(NullNode),
    Number(NumberNode),
    Object(ObjectNode),
    String(StringNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectNode {
    pub object_pair: Vec<ObjectPairNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectPairNode {
    pub object_key: ObjectKeyNode,
    pub value: ValueNode,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectKeyNode {
    Identifier(IdentifierNode),
    Integer(IntegerNode),
    String(StringNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayNode {
    pub value: Vec<ValueNode>,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StringNode {
    String0(StringRawNode),
    String1(StringTextNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringRawNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StringTextNode {
    StringEscape(StringEscapeNode),
    StringText1,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringEscapeNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntegerNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BooleanNode {
    Boolean0,
    Boolean1,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NullNode {
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhiteSpaceNode {
    pub span: Range<u32>,
}

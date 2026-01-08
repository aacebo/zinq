use zinq_token::{Comma, DotDot, Ident, LBrace, Punctuated, RBrace};

use crate::TypePath;

///
/// ## Struct Pattern
/// `MyStruct { a, b, .. }`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructPattern {
    pub path: TypePath,
    pub left_brace: LBrace,
    pub fields: Punctuated<Ident, Comma>,
    pub spread: Option<DotDot>,
    pub right_brace: RBrace,
}

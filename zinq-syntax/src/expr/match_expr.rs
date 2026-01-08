use zinq_token::{LBrace, Match, RBrace};

use crate::{expr::Expr};

///
/// ## Match Expression
/// `match <expr>`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchExpr {
    pub keyword: Match,
    pub expr: Box<Expr>,
    pub left_brace: LBrace,

    pub right_brace: RBrace,
}

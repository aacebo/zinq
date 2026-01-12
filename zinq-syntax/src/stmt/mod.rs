mod block_stmt;
mod enum_stmt;
mod expr_stmt;
mod fn_stmt;
mod for_stmt;
mod if_stmt;
mod impl_stmt;
mod let_stmt;
mod mod_stmt;
mod parser;
mod return_stmt;
mod struct_stmt;
mod use_stmt;

pub use block_stmt::*;
pub use enum_stmt::*;
pub use expr_stmt::*;
pub use fn_stmt::*;
pub use for_stmt::*;
pub use if_stmt::*;
pub use impl_stmt::*;
pub use let_stmt::*;
pub use mod_stmt::*;
pub use parser::*;
pub use return_stmt::*;
pub use struct_stmt::*;
pub use use_stmt::*;

use zinq_error::Result;
use zinq_parse::{Parse, Peek, Spanned};

use crate::Node;

///
/// ## Statement
/// Something that can appear as a “line” inside a block
/// and doesn’t itself produce a value you can use in
/// a larger expression.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    Block(BlockStmt),
    Expr(ExprStmt),
    Let(LetStmt),
    Mod(ModStmt),
    Struct(StructStmt),
    Fn(FnStmt),
    Impl(ImplStmt),
    Use(UseStmt),
    Return(ReturnStmt),
    If(IfStmt),
    Enum(EnumStmt),
    For(ForStmt),
}

impl Stmt {
    pub fn is_block(&self) -> bool {
        match self {
            Self::Block(_) => true,
            _ => false,
        }
    }

    pub fn is_expr(&self) -> bool {
        match self {
            Self::Expr(_) => true,
            _ => false,
        }
    }

    pub fn is_let(&self) -> bool {
        match self {
            Self::Let(_) => true,
            _ => false,
        }
    }

    pub fn is_mod(&self) -> bool {
        match self {
            Self::Mod(_) => true,
            _ => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Self::Struct(_) => true,
            _ => false,
        }
    }

    pub fn is_fn(&self) -> bool {
        match self {
            Self::Fn(_) => true,
            _ => false,
        }
    }

    pub fn is_impl(&self) -> bool {
        match self {
            Self::Impl(_) => true,
            _ => false,
        }
    }

    pub fn is_use(&self) -> bool {
        match self {
            Self::Use(_) => true,
            _ => false,
        }
    }

    pub fn is_return(&self) -> bool {
        match self {
            Self::Return(_) => true,
            _ => false,
        }
    }

    pub fn is_if(&self) -> bool {
        match self {
            Self::If(_) => true,
            _ => false,
        }
    }

    pub fn is_enum(&self) -> bool {
        match self {
            Self::Enum(_) => true,
            _ => false,
        }
    }

    pub fn is_for(&self) -> bool {
        match self {
            Self::For(_) => true,
            _ => false,
        }
    }

    pub fn as_block(&self) -> &BlockStmt {
        match self {
            Self::Block(v) => v,
            v => panic!("expected BlockStmt, received {}", v.name()),
        }
    }

    pub fn as_expr(&self) -> &ExprStmt {
        match self {
            Self::Expr(v) => v,
            v => panic!("expected ExprStmt, received {}", v.name()),
        }
    }

    pub fn as_let(&self) -> &LetStmt {
        match self {
            Self::Let(v) => v,
            v => panic!("expected LetStmt, received {}", v.name()),
        }
    }

    pub fn as_mod(&self) -> &ModStmt {
        match self {
            Self::Mod(v) => v,
            v => panic!("expected ModStmt, received {}", v.name()),
        }
    }

    pub fn as_struct(&self) -> &StructStmt {
        match self {
            Self::Struct(v) => v,
            v => panic!("expected StructStmt, received {}", v.name()),
        }
    }

    pub fn as_fn(&self) -> &FnStmt {
        match self {
            Self::Fn(v) => v,
            v => panic!("expected FnStmt, received {}", v.name()),
        }
    }

    pub fn as_impl(&self) -> &ImplStmt {
        match self {
            Self::Impl(v) => v,
            v => panic!("expected ImplStmt, received {}", v.name()),
        }
    }

    pub fn as_use(&self) -> &UseStmt {
        match self {
            Self::Use(v) => v,
            v => panic!("expected UseStmt, received {}", v.name()),
        }
    }

    pub fn as_return(&self) -> &ReturnStmt {
        match self {
            Self::Return(v) => v,
            v => panic!("expected ReturnStmt, received {}", v.name()),
        }
    }

    pub fn as_if(&self) -> &IfStmt {
        match self {
            Self::If(v) => v,
            v => panic!("expected IfStmt, received {}", v.name()),
        }
    }

    pub fn as_enum(&self) -> &EnumStmt {
        match self {
            Self::Enum(v) => v,
            v => panic!("expected EnumStmt, received {}", v.name()),
        }
    }

    pub fn as_for(&self) -> &ForStmt {
        match self {
            Self::For(v) => v,
            v => panic!("expected ForStmt, received {}", v.name()),
        }
    }
}

impl Node for Stmt {
    fn name(&self) -> &str {
        match self {
            Self::Block(v) => v.name(),
            Self::Expr(v) => v.name(),
            Self::Let(v) => v.name(),
            Self::Mod(v) => v.name(),
            Self::Struct(v) => v.name(),
            Self::Fn(v) => v.name(),
            Self::Impl(v) => v.name(),
            Self::Use(v) => v.name(),
            Self::Return(v) => v.name(),
            Self::If(v) => v.name(),
            Self::Enum(v) => v.name(),
            Self::For(v) => v.name(),
        }
    }
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block(v) => write!(f, "{}", v),
            Self::Expr(v) => write!(f, "{}", v),
            Self::Let(v) => write!(f, "{}", v),
            Self::Mod(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
            Self::Fn(v) => write!(f, "{}", v),
            Self::Impl(v) => write!(f, "{}", v),
            Self::Use(v) => write!(f, "{}", v),
            Self::Return(v) => write!(f, "{}", v),
            Self::If(v) => write!(f, "{}", v),
            Self::Enum(v) => write!(f, "{}", v),
            Self::For(v) => write!(f, "{}", v),
        }
    }
}

impl Peek for Stmt {
    fn peek(_: &zinq_parse::Cursor, _: &zinq_parse::ZinqParser) -> Result<bool> {
        Ok(true)
    }
}

impl Parse for Stmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        parser.parse_stmt(cursor)
    }
}

impl Spanned for Stmt {
    fn span(&self) -> zinq_parse::Span {
        match self {
            Self::Use(v) => v.span(),
            Self::Block(v) => v.span(),
            Self::Expr(v) => v.span(),
            Self::Let(v) => v.span(),
            Self::Struct(v) => v.span(),
            Self::Mod(v) => v.span(),
            Self::Fn(v) => v.span(),
            Self::Impl(v) => v.span(),
            Self::Return(v) => v.span(),
            Self::If(v) => v.span(),
            Self::Enum(v) => v.span(),
            Self::For(v) => v.span(),
        }
    }
}

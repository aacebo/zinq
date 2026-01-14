mod group_pattern;
mod literal_pattern;
mod or_pattern;
mod parser;
mod path_pattern;
mod ref_pattern;
mod spread_pattern;
mod struct_pattern;
mod tuple_pattern;
mod visitor;
mod wild_pattern;

use std::any::type_name_of_val;

pub use group_pattern::*;
pub use literal_pattern::*;
pub use or_pattern::*;
pub use parser::*;
pub use path_pattern::*;
pub use ref_pattern::*;
pub use spread_pattern::*;
pub use struct_pattern::*;
pub use tuple_pattern::*;
pub use visitor::*;
pub use wild_pattern::*;

use zinq_parse::{Parse, Peek, Spanned};

use crate::Syntax;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    Wild(WildPattern),
    Path(PathPattern),
    Literal(LiteralPattern),
    Ref(RefPattern),
    Spread(SpreadPattern),
    Struct(StructPattern),
    Tuple(TuplePattern),
    Group(GroupPattern),
    Or(OrPattern),
}

impl Pattern {
    pub fn is_wild(&self) -> bool {
        match self {
            Self::Wild(_) => true,
            _ => false,
        }
    }

    pub fn is_path(&self) -> bool {
        match self {
            Self::Path(_) => true,
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            Self::Literal(_) => true,
            _ => false,
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            Self::Ref(_) => true,
            _ => false,
        }
    }

    pub fn is_spread(&self) -> bool {
        match self {
            Self::Spread(_) => true,
            _ => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Self::Struct(_) => true,
            _ => false,
        }
    }

    pub fn is_tuple(&self) -> bool {
        match self {
            Self::Tuple(_) => true,
            _ => false,
        }
    }

    pub fn is_group(&self) -> bool {
        match self {
            Self::Group(_) => true,
            _ => false,
        }
    }

    pub fn is_or(&self) -> bool {
        match self {
            Self::Or(_) => true,
            _ => false,
        }
    }

    pub fn as_wild(&self) -> &WildPattern {
        match self {
            Self::Wild(v) => v,
            v => panic!("expected WildPattern, received {}", type_name_of_val(v)),
        }
    }

    pub fn as_path(&self) -> &PathPattern {
        match self {
            Self::Path(v) => v,
            v => panic!("expected PathPattern, received {}", type_name_of_val(v)),
        }
    }

    pub fn as_literal(&self) -> &LiteralPattern {
        match self {
            Self::Literal(v) => v,
            v => panic!("expected LiteralPattern, received {}", type_name_of_val(v)),
        }
    }

    pub fn as_ref(&self) -> &RefPattern {
        match self {
            Self::Ref(v) => v,
            v => panic!("expected RefPattern, received {}", type_name_of_val(v)),
        }
    }

    pub fn as_spread(&self) -> &SpreadPattern {
        match self {
            Self::Spread(v) => v,
            v => panic!("expected SpreadPattern, received {}", type_name_of_val(v)),
        }
    }

    pub fn as_struct(&self) -> &StructPattern {
        match self {
            Self::Struct(v) => v,
            v => panic!("expected StructPattern, received {}", type_name_of_val(v)),
        }
    }

    pub fn as_tuple(&self) -> &TuplePattern {
        match self {
            Self::Tuple(v) => v,
            v => panic!("expected TuplePattern, received {}", type_name_of_val(v)),
        }
    }

    pub fn as_group(&self) -> &GroupPattern {
        match self {
            Self::Group(v) => v,
            v => panic!("expected GroupPattern, received {}", type_name_of_val(v)),
        }
    }

    pub fn as_or(&self) -> &OrPattern {
        match self {
            Self::Or(v) => v,
            v => panic!("expected OrPattern, received {}", type_name_of_val(v)),
        }
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wild(v) => write!(f, "{}", v),
            Self::Path(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::Ref(v) => write!(f, "{}", v),
            Self::Spread(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
            Self::Tuple(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
            Self::Or(v) => write!(f, "{}", v),
        }
    }
}

impl Spanned for Pattern {
    fn span(&self) -> zinq_parse::Span {
        match self {
            Self::Wild(v) => v.span(),
            Self::Path(v) => v.span(),
            Self::Literal(v) => v.span(),
            Self::Ref(v) => v.span(),
            Self::Spread(v) => v.span(),
            Self::Struct(v) => v.span(),
            Self::Tuple(v) => v.span(),
            Self::Group(v) => v.span(),
            Self::Or(v) => v.span(),
        }
    }
}

impl Peek for Pattern {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for Pattern {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        parser.parse_pattern(cursor)
    }
}

impl Syntax for Pattern {
    fn name(&self) -> &str {
        match self {
            Self::Group(v) => v.name(),
            Self::Literal(v) => v.name(),
            Self::Or(v) => v.name(),
            Self::Path(v) => v.name(),
            Self::Ref(v) => v.name(),
            Self::Spread(v) => v.name(),
            Self::Struct(v) => v.name(),
            Self::Tuple(v) => v.name(),
            Self::Wild(v) => v.name(),
        }
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_pattern(self);

        match self {
            Self::Group(v) => v.accept(visitor),
            Self::Literal(v) => v.accept(visitor),
            Self::Or(v) => v.accept(visitor),
            Self::Path(v) => v.accept(visitor),
            Self::Ref(v) => v.accept(visitor),
            Self::Spread(v) => v.accept(visitor),
            Self::Struct(v) => v.accept(visitor),
            Self::Tuple(v) => v.accept(visitor),
            Self::Wild(v) => v.accept(visitor),
        }
    }
}

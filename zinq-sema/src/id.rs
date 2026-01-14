use std::hash::{Hash, Hasher};

macro_rules! define_id {
    ($name:ident => $src:path) => {
        #[derive(Debug, Default, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
        pub struct $name(u64);

        impl From<$src> for $name {
            fn from(value: $src) -> Self {
                let mut hasher = std::hash::DefaultHasher::new();
                value.hash(&mut hasher);
                Self(hasher.finish())
            }
        }

        impl std::ops::Deref for $name {
            type Target = u64;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &self.0)
            }
        }
    };
}

define_id!(ExprId => zinq_syntax::expr::Expr);
define_id!(StmtId => zinq_syntax::stmt::Stmt);

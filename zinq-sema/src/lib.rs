pub mod scope;

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(u32);

        impl From<u32> for $name {
            fn from(value: u32) -> Self {
                Self(value)
            }
        }

        impl std::ops::Deref for $name {
            type Target = u32;

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

define_id!(ScopeId);
define_id!(ExprId);
define_id!(StmtId);
define_id!(SymbolId);

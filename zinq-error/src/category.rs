///
/// ## ErrorCategory
/// an error category definition
/// ### Examples
/// `Compiler`, `Parser`, etc...
///
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ErrorCategory {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

///
/// ## Severity
/// the severity level of a `Diagnostic`
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    ///
    /// ## Error
    /// The code is not acceptable under the rules; compilation/analysis for that unit is considered failed.
    /// ### Behavior
    /// Fails the build (or at least prevents emitting an artifact), though compilers still try to keep going to find more errors.
    /// ### Examples
    /// syntax error, type mismatch, unresolved name.
    ///
    Error,

    ///
    /// ## Warning
    /// The code is valid, but there’s likely a bug, foot-gun, or maintainability issue.
    /// ### Behavior
    /// Does not fail the build by default, but can be promoted to error (e.g., “treat warnings as errors”).
    /// ### Examples
    /// unused variable, deprecated API usage, unreachable code.
    ///
    Warning,

    ///
    /// ## Note
    /// Additional context that explains an error/warning—not a standalone problem.
    /// ### Behavior
    /// Shown attached to a parent diagnostic. Often points to a related location.
    /// ### Examples
    /// “`x` is defined here”, “required by this bound”, “in expansion of macro …”.
    ///
    Note,

    ///
    /// ## Help
    /// A suggestion or actionable guidance to resolve the parent diagnostic.
    /// ### Behavior
    /// Also attached to a parent; sometimes includes a fix-it.
    /// ### Examples
    /// “consider adding `mut`”, “try importing `Foo`”, “use `&` to borrow”.
    ///
    Help,
}

impl Severity {
    pub fn prefix(&self) -> char {
        match self {
            Self::Error => 'E',
            Self::Warning => 'W',
            Self::Note => 'N',
            Self::Help => 'H',
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warn"),
            Self::Note => write!(f, "note"),
            Self::Help => write!(f, "help"),
        }
    }
}

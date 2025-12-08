#[macro_export]
macro_rules! get {
    // Entry: start with an immutable borrow of the root expression
    ($root:expr => $($path:tt)+) => {
        $crate::get!(@acc &($root) ; $($path)+)
    };

    // The path must start with a slash: /seg/seg/...
    (@acc $acc:expr ; / $($rest:tt)+ ) => {
        $crate::get!(@seg $acc ; $($rest)+)
    };

    // --- identifier segment (struct/tuple-struct field) ---
    // More segments after this one
    (@seg $acc:expr ; $field:ident / $($rest:tt)+ ) => {
        $crate::get!(@acc ::core::ops::Index::index($acc, stringify!($field)) ; / $($rest)+ )
    };
    // Terminal identifier segment
    (@seg $acc:expr ; $field:ident ) => {
        ::core::ops::Index::index($acc, stringify!($field))
    };

    // --- literal segment (numbers, strings, etc.) using Index ---
    // More segments after this one
    (@seg $acc:expr ; $idx:literal / $($rest:tt)+ ) => {
        $crate::get!(@acc ::core::ops::Index::index($acc, $idx) ; / $($rest)+ )
    };
    // Terminal literal segment
    (@seg $acc:expr ; $idx:literal ) => {
        ::core::ops::Index::index($acc, $idx)
    };
}
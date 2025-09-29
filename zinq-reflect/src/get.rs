#[macro_export]
macro_rules! get {
    // Entry: start with an immutable borrow of the root expression
    ($root:expr => $($path:tt)+) => {
        get!(@acc &($root) ; $($path)+)
    };

    // The path must start with a slash: /seg/seg/...
    (@acc $acc:expr ; / $($rest:tt)+ ) => {
        get!(@seg $acc ; $($rest)+)
    };

    // --- identifier segment (struct/tuple-struct field) ---
    // More segments after this one
    (@seg $acc:expr ; $field:ident / $($rest:tt)+ ) => {
        get!(@acc ::core::ops::Index::index($acc, stringify!($field)) ; / $($rest)+ )
    };
    // Terminal identifier segment
    (@seg $acc:expr ; $field:ident ) => {
        ::core::ops::Index::index($acc, stringify!($field))
    };

    // --- literal segment (numbers, strings, etc.) using Index ---
    // More segments after this one
    (@seg $acc:expr ; $idx:literal / $($rest:tt)+ ) => {
        get!(@acc ::core::ops::Index::index($acc, $idx) ; / $($rest)+ )
    };
    // Terminal literal segment
    (@seg $acc:expr ; $idx:literal ) => {
        ::core::ops::Index::index($acc, $idx)
    };
}

#[macro_export]
macro_rules! get_mut {
    // Entry: start with an immutable borrow of the root expression
    ($root:expr => $($path:tt)+) => {
        get_mut!(@acc &mut ($root) ; $($path)+)
    };

    // The path must start with a slash: /seg/seg/...
    (@acc $acc:expr ; / $($rest:tt)+ ) => {
        get_mut!(@seg $acc ; $($rest)+)
    };

    // --- identifier segment (struct/tuple-struct field) ---
    // More segments after this one
    (@seg $acc:expr ; $field:ident / $($rest:tt)+ ) => {
        get_mut!(@acc ::core::ops::IndexMut::index_mut($acc, stringify!($field)) ; / $($rest)+ )
    };
    // Terminal identifier segment
    (@seg $acc:expr ; $field:ident ) => {
        ::core::ops::IndexMut::index_mut($acc, stringify!($field))
    };

    // --- literal segment (numbers, strings, etc.) using Index ---
    // More segments after this one
    (@seg $acc:expr ; $idx:literal / $($rest:tt)+ ) => {
        get_mut!(@acc ::core::ops::IndexMut::index_mut($acc, $idx) ; / $($rest)+ )
    };
    // Terminal literal segment
    (@seg $acc:expr ; $idx:literal ) => {
        ::core::ops::IndexMut::index_mut($acc, $idx)
    };
}

#[cfg(test)]
mod test {
    use crate::value_of;

    #[test]
    pub fn basic() {
        let meta = crate::MetaData::from([
            ("a", value_of!(21)),
            ("b", value_of!(true)),
            ("c", value_of!([3, 2, 1])),
        ]);

        let out = get!(meta => /c/1);
        assert_eq!(out, &value_of!(2));
    }

    #[test]
    pub fn mutable() {
        let mut meta = crate::MetaData::from([
            ("a", value_of!(21)),
            ("b", value_of!(true)),
            ("c", value_of!([3, 2, 1])),
        ]);

        let out = get_mut!(meta => /c/1);
        assert_eq!(out, &value_of!(2));
    }
}

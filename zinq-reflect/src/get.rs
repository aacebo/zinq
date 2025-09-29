#[macro_export]
macro_rules! get {
    ($target:expr => $(/$item:tt)*) => {{
        let mut out = $crate::value_of!($target.clone());
        $(out = get!(out => $item);)*
        out
    }};
    [$target:expr => $item:literal] => {
        ::std::ops::Index::index(&$target, &$crate::value_of!($item)).clone()
    };
    [$target:expr => $item:ident] => {
        ::std::ops::Index::index(&$target, stringify!($item)).clone()
    };
}

#[cfg(test)]
mod test {
    use crate::{get, value_of};

    #[test]
    pub fn basic() {
        let meta = crate::MetaData::from([
            ("a", value_of!(21)),
            ("b", value_of!(true)),
            ("c", value_of!([3, 2, 1])),
        ]);

        let out = get!(meta => /c/1);
        assert_eq!(out, value_of!(2));
    }
}

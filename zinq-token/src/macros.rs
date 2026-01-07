#[macro_export]
macro_rules! Stream {
    ($($body:tt)*) => {{
        <$crate::TokenStream as std::str::FromStr>::from_str(stringify!($($body)*))
    }};
}

#[cfg(test)]
mod test {
    use zinq_error::Result;

    #[test]
    fn should_tokenize_pattern() -> Result<()> {
        let mut stream = Stream! {
            fn my_test_function() -> string {
                return "hello, world!";
            }
        }?;

        debug_assert_eq!(stream.len(), 11);
        stream = Stream!(let test: u16 = 500u16)?;
        debug_assert_eq!(stream.len(), 6);

        Ok(())
    }
}

# ErrorX

error and result types/utilities.

## Code

```rust
#[derive(Error)]
pub enum MyError {
    #[error(code = 1023, message = "an error has occurred => {message}")]
    Basic {
        message: String,
    },
}
```

## Result

```rust
impl zinq::error::ToError for MyError {
    fn to_error(&self) -> zinq::error::Error {
        return match self {
            Self::Basic { message } => {
                zinq::error::Error::new()
                    .with_code(1023)
                    .with_message(format!("an error has occurred => {message}", message = message))
                    .build()
            },
        };
    }
}
```
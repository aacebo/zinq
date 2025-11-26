# Zinq Parse (Macros)

macro type parsing helpers

```rust
pub struct MyElement;

impl MyElement {
    #[render]
    pub fn render(&self, context: &mut MyContext) -> Result<TokenStream, Error> {
        ...
    }
}
```
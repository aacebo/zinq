# Zinq Schema

the core schema abstractions for zinq crates.

```rust
#[schema("user")]
pub struct User {
    #[field("id")]
    pub id: uuid::Uuid,

    #[field("name")]
    pub name: String,
}
```
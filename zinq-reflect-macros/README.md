# Zinq Reflect (Macros)

macros for the zinq-reflect crate.

```rust
#[derive(Reflect)]
enum Kind {
    Admin,
    Moderator,
    Basic,
}

#[derive(Reflect)]
struct User {
    pub kind: Kind,
    pub name: String,
    pub password: String,
}
```
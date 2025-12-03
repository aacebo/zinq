# ExtendX

type and implementation extentions

## Usage

### Code

```rust
pub struct A {
    a: usize,
}

#[derive(Extend)]
pub struct B {
    #[extend]
    a: A,
    b: bool,
}
```

### Expanded

```rust
pub struct A {
    a: usize,
}

#[derive(Extend)]
pub struct B {
    #[extend]
    a: A,
    b: bool,
}

impl B {
    pub fn a(&self) -> &usize {
        return &self.a.a;
    }
}
```
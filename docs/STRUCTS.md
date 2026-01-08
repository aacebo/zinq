# Zinq: Structs

```
struct A {
    a: string,
}

// B has all the fields from A
pub struct B {
    ..A,
    b: int,
}

// C has all the fields from B
pub struct C {
    ..B,
    c: bool,
}
```
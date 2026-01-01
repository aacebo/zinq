# Zinq: Functions

## Basic

```
fn hello_world() -> string {
    "hello, world"
}
```

> The lack of `pub` means this method is private.

## Parameters

### Immutable

```
pub fn get_string_length(value: &string) -> uint {
    return value.len();
}
```

> Notice you can return from a function with or without the `return`
> keyword.

### Mutable

```
pub fn append_to_string(value: &mut string, other: &string) {
    value += other;
}
```

> Notice the mutability of the first parameter.
> Parameters must be marked explicitly as mutable for them to
> be mutated or assigned to.
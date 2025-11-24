# To Do

- [zinq-reflect](./zinq-reflect/TODO.md)
- [zinq-error]
    - hook into `std::backtrace::Backtrace`
    - add `#[backtrace]` macro
    - error message templates
    - implement `From` for common error types:
        - `HttpError`
        - `std::fmt::Error`
        - `std::io::Error`
        - `std::syn::Error`
        - etc...
- [zinq-extend]
    - struct extend struct fields/methods
        - downcast methods from child to parent type
        `to_${parent}(&self) -> ${Parent}`
        - from methods for parent from child type
        `impl From<Child> for Parent`
    - enum extend enum variants/methods
    - if all enum variants have a method/field, auto
    generate forward methods using `match`
    - extension traits for external types
        - ie `impl From<&[T]> for Vec<T>`
    - `#[proxy(${field})]` to add all the methods from the wrapped
    type to your type and proxy their calls to the inner field
- [zinq-parse]
    - `TypeRegistry` for building shared macro type state
    - `#[import]` macro at root to recursively build `TypeRegistry`
- [zinq-builder]
    - `#[derive(Build)]` for auto generation of builder struct
- [`??`]
    -  deconstruct named fields to tuple
    ```rust
    #[derive(Tuple)]
    struct MyStruct {
        a: usize,
        b: usize,
        c: usize,
    }

    let value = MyStruct {
        a: 1,
        b: 2,
        c: 3,
    }

    let (a, b, c) = &value.as_tuple();
    ```
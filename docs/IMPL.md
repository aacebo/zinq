# Zinq: Impl

Impl blocks add new functionality to an existing type.
This includes the ability to extend types for which you don’t have access to the original source code.

## Struct

```
struct User {
    name: string,
    dob: Date,
}

impl User {
    pub fn stringify(&self) -> string {
        "{self.name()}: {self.dob()}"
    }
}
```

## Protocol

```
protocol User {
    fn name(&self) -> &string;
    fn dob(&self) -> &Date;
}

impl User {
    pub fn stringify(&self) -> string {
        "{self.name()}: {self.dob()}"
    }
}
```

## External Type

```
impl string {
    pub fn is_empty(self) -> bool {
        self == ""
    }
}
```
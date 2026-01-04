# Zinq: Macros

## Struct

```
#[Clone, ToString]
pub struct Message {
    pub text: string,
}
```

## Method

```
#[http::post("/api/messages")]
pub fn on_message() {
    ...
}
```
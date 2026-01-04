# Zinq: Macros

## Struct

```
#[Clone, ToString]
pub struct Message {
    pub text: string,
}
```

## Method

> Feature flagged http handler for post requests

```
#[if = feature(http), http::post("/api/messages")]
pub fn on_message() {
    ...
}
```
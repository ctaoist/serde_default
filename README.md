## Overview

Add `#[serde(default)]` to the every field of struct.

There is no need adding manually `#[serde(default)]` to fields of struct when deserializing a json string missing some fields.

```rust
use serde_default::serde_default;
use serde::{Deserialize, Serialize};

#[serde_default] // this needs to be above of #[derive(Serialize...)]
#[derive(Deserialize, Serialize)]
Struct Foo {
  name: String,
  age: int,
  #[serde(alias = "foobar", default)]
  bar: String,
}
```

The result that the macro expanded is:

```rust
#[derive(Deserialize, Serialize)]
struct Foo {
  #[serde(default)]
  name: String,
  #[serde(default)]
  age: usize,
  #[serde(alias = "foobar", default)]
  bar: String,
}
```

usage:

```rust
fn main() {
  let john = json!({"name": "lyf"});
  let p: Foo = serde_json::from_str(&john.to_string()).unwrap();
}

// the program would not panic because of missing field `age`.
```

## Usage

`Cargo.toml`:

```toml
...

[dependencies]
serde_default = {git = "https://github.com/ctaoist/serde_default"}
serde = { version = "*", features = ["derive"] }
```
A lightweight wrapper around [`std::env::var`], with more convenient common operations and errors.

# Install

`Cargo.toml`:

```toml
[dependencies]
env-var-helpers = { git = "https://github.com/PiDelport/rust-env-var-helpers" }
```

# Usage

```rust
use env_var_helpers::env_vars;

fn main() {
    let api_key = env_vars::var("API_KEY");

    // Defaults:
    let bind_addr = env_vars::var_default("BIND_ADDR", "127.0.0.1:8080");

    // Errors name the environment variable:
    assert_eq!(
        api_key.unwrap_err().to_string(),
        "API_KEY: environment variable not found"
    );
}

// Errors automatically convert to io::Error:

use std::io;

fn run() -> io::Result<()> {
    let path = env_vars::var("PATH")?;

    todo!()
}
```

## Comparison with [`std::env::var`]

```rust
use std::env;

fn main() {
    let api_key = env::var("API_KEY");

    // Inconvenient and error-prone defaults:
    let default_addr = "127.0.0.1:8080".to_string();
    let bind_addr = match env::var("BIND_ADDR") {
        Err(env::VarError::NotPresent) => Ok(default_addr),
        Ok(value) if value.is_empty() => Ok(default_addr),
        otherwise => otherwise,
    };

    // Errors don't name the environment variable:
    assert_eq!(
        api_key.unwrap_err().to_string(),
        "environment variable not found"
    );
}

// Errors require manual conversion to io::Error:

use std::io;

fn run() -> io::Result<()> {
    let path = env::var("PATH").map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

    todo!()
}
```

[`std::env::var`]: https://doc.rust-lang.org/std/env/fn.var.html

# More Options &emsp; ![CI][ci-badge] [![Crates.io][crates-badge]][crates-url] [![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/more-options.svg
[crates-url]: https://crates.io/crates/more-options
[mit-badge]: https://img.shields.io/badge/license-MIT-blueviolet.svg
[mit-url]: https://github.com/commonsensesoftware/more-rs-options/blob/main/LICENSE
[ci-badge]: https://github.com/commonsensesoftware/more-rs-options/actions/workflows/ci.yml/badge.svg

More Options is a library for defining configuration options in Rust. Options can be initialized in code, bound from
configuration, and/or composed through dependency injection (DI).

You may be looking for:

- [User Guide](https://commonsensesoftware.github.io/more-rs-options)
- [API Documentation](https://docs.rs/more-options)
- [Release Notes](https://github.com/commonsensesoftware/more-rs-options/releases)

## Features

This crate provides the following features:

- _default_ - Abstractions for options
- **async** - Enable options in asynchronous contexts
- **di** - Dependency injection extensions
- **cfg** - Dependency injection extensions to bind configurations to options

## Options Pattern

The options pattern uses structures to provide strongly typed access to groups of related settings without having to
know how the settings were configured. The settings can be set explicitly in code or they can come from an external
configuration source such as a file.

Consider the following options:

```rust
pub struct EndpointOptions {
    pub url: String,
    pub retries: usize,
}
```

These might be used by a HTTP client as follows:


```rust
use options::Options;
use std::rc::Rc;

pub struct HttpClient {
    options: Rc<EndpointOptions>,
}

impl HttpClient {
    pub fn new(options: Rc<EndpointOptions>) -> Self {
        Self { options }
    }

    pub fn retries(&self) -> usize {
        self.options.value().retries
    }
}
```

# Options in Action

The defined options can be used in any number of ways, including just explicitly specifying the settings.

```rust
use crate::{EndpointOptions, HttpClient};
use std::rc::Rc;

fn main() {
    let options = Rc::new(EndpointOptions {
        url: "https://tempuri.org",
        retries: 2,
    });
    let client = HttpClient::new(options);
    
    // TODO: use the client
}
```

If you expect to process your options from an external data source, then you'll almost certainly require supporting
deserialization using [serde](https://crates.io/crates/serde) as follows:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EndpointOptions {
    pub url: String,
    pub retries: usize,
}
```

Suppose you had the following `appSettings.json` file:

```json
{
  "url": "https://tempuri.org",
  "retries": 3
}
```

You can construct the options from the settings by including the [more-config](https://crates.io/crates/more-config)
crate as follows:

```rust
use crate::{EndpointOptions, HttpClient};
use config::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let config = config::builder().add_json_file("appsettings.json").build()?;
    let options: EndpointOptions = config.reify()?;
    let client = HttpClient::new(options);

    // TODO: use the client

    Ok(())
}
```

You can go one step further and combine that configuration with the [more-di](https://crates.io/crates/more-di) crate
to assemble all of the pieces for you:

```rust
use crate::{EndpointOptions, HttpClient};
use config::prelude::*;
use di::ServiceCollection;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let config = config::builder().add_json_file("appsettings.json").build()?;
    let provider = ServiceCollection::new()
        .add(transient_as_self::<HttpClient>())
        .apply_config::<EndpointOptions>(config)
        .build_provider()?;
    let client = provider.get_required::<HttpClient>();

    // TODO: use the client

    Ok(())
}
```

## Examples

### Basic

A basic demonstration application is provided that combines in-memory settings resolves them as a set of _options_.
Run it with:

```bash
cargo run --example basic
```

### Dependency Injection

A variant of the demonstration application is provided that illustrates how to provide composition, which removes the
setup ceremony, using dependency injection (DI). Run it with:

```bash
cargo run --example di
```

## Minimum Supported Rust Version

When increasing the minimum supported Rust version (MSRV), the new version must have been released at least six months
ago. The current MSRV is 1.79.


## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/commonsensesoftware/more-rs-options/blob/main/LICENSE
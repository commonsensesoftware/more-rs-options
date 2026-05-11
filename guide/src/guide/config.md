{{#include links.md}}

# Configuration Binding

The preferred way to read related configuration values is using the options pattern. For example, to read the
following configuration values:

```json
{
  "Position": {
    "Title": "Editor",
    "Name": "Joe Smith"
  }
}
```

Create the following `PositionOptions` struct:

```rust
#[derive(Default)]
pub struct PositionOptions {
    pub title: String,
    pub name: String,
}
```

An options struct:

- must be public.
- should implement the `Default` trait; otherwise a custom options [Factory] is required.
- binds public read-write fields.

The following code:

- calls [Binder::bind] to bind the `PositionOptions` class to the `"Position"` section.
- displays the `Position` configuration data.
- requires the **binder** feature to be enabled
  - which transitively enables the **serde** feature

```rust
use config::prelude::*;
use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct PositionOptions {
    pub title: String,
    pub name: String,
}

pub TestModel<'a> {
    config: &'a dyn Configuration
}

impl<'a> TestModel<'a> {
    pub new(config: &dyn Configuration) -> Self {
        Self { config }
    }

    pub get(&self) -> String {
        let mut options = PositionOptions::default();
        let section = self.config.section("Position").bind_unchecked(&mut options);
        format!("Title: {}\nName: {}", options.title, options.name)
    }
}
```

[Binder::reify] binds and returns the specified type. [Binder::reify] may be more convenient than using
[Binder::bind]. The following code shows how to use [Binder::reify] with the `PositionOptions` struct:

```rust
use config::prelude::*;

pub TestModel<'a> {
  config: &'a dyn Configuration
}

impl<'a> TestModel<'a> {
  pub new(config: &dyn Configuration) -> Self {
    Self { config }
  }

  pub get(&self) -> String {
    let options: PositionOptions = self.config.section("Position").reify_unchecked();
    format!("Title: {}\nName: {}", options.title, options.name)
  }
}
```

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
- should implement the `Default` trait; otherwise a custom `OptionsFactory<TOptions>` is required.
- binds public read-write fields.

The following code:

- calls `ConfigurationBinder::bind` to bind the `PositionOptions` class to the `"Position"` section.
- displays the `Position` configuration data.
- requires the **binder** feature to be enabled
  - which transitively enables the **serde** feature

```rust
use config::*;

#[derive(Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
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
        let section = self.config.section("Position").bind(&mut options);
        format!("Title: {}\nName: {}", options.title, options.name)
    }
}
```

`ConfigurationBinder::reify<T>` binds and returns the specified type. `ConfigurationBinder::reify<T>` may be more convenient than using `ConfigurationBinder::bind`. The following code shows how to use `ConfigurationBinder::reify<T>` with the `PositionOptions` struct:

```rust
use config::*;

pub TestModel<'a> {
    config: &'a dyn Configuration
}

impl<'a> TestModel<'a> {
    pub new(config: &dyn Configuration) -> Self {
        Self { config }
    }

    pub get(&self) -> String {
        let options: PositionOptions = self.config.section("Position").reify();
        format!("Title: {}\nName: {}", options.title, options.name)
    }
}
```

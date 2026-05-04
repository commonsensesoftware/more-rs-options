{{#include links.md}}

# Validation

Options validation enables configured option values to be validated. Validation is performed via [ValidateOptions],
which is typically invoked during options construction through [OptionsFactory] rather than imperatively.

Consider the following `appsettings.json` file:

```json
{
  "MyConfig": {
    "Key1": "My Key One",
    "Key2": 10,
    "Key3": 32
  }
}
```

The application settings might be bound to the following options struct:

```rust
#[derive(Default, Deserialize)]
pub struct MyConfigOptions {
    pub key1: String,
    pub key2: usize,
    pub key3: usize,
}
```

The following code:

- uses dependency injection (DI).
- calls [add_options] to get an [OptionsBuilder] that binds to the `MyConfigOptions` struct.
- invokes a closure to validate the struct.

```rust
use config::prelude::*;
use di::ServiceCollection;
use options::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let config = config::builder().add_json_file("appsettings.json").build()?;
    let provider = ServiceCollection::new()
        .apply_config_at::<MyConfigOptions>(config, "MyConfig")
        .validate(
            |options| options.key2 == 0 || options.key3 > options.key2,
            "Key3 must be > than Key2.")
        .build_provider()?;

    Ok(())
}
```

Dependency injection is not required to enforce validation, but it is the simplest and fastest way to compose all of the
necessary pieces together.

## Implementing `ValidateOptions`

[ValidateOptions] enables moving the validation code out of a closure and into a struct. The following struct implements
[ValidateOptions]:

```rust
use di::injectable;
use options::{ValidationOptions, ValidationOptionsResult};

#[injectable(ValidationOptions<MyConfigOptions>)]
struct MyConfigValidation;

impl ValidationOptions<MyConfigOptions> for MyConfigValidation {
    fn validate(
        &self,
        name: Option<&str>,
        options: &MyConfigOptions) -> ValidateOptionsResult
    {
        let failures = Vec::default();

        if options.key2 < 0 || options.key2 > 1000 {
            failures.push(format!("{} doesn't match Range 0 - 1000", options.key2));
        }

        if config.key3 <= config.key2 {
            failures.push("Key3 must be > than Key2");
        }

        if failures.is_empty() {
            ValidationOptionsResult::success()
        } else {
            ValidationOptionsResult::fail_many(failures)
        }
    }
}
```

Using the preceding code, validation is enabled with the following code:

```rust
use config::prelude::*;
use di::ServiceCollection;
use options::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let config = config::builder().add_json_file("appsettings.json").build()?;
    let provider = ServiceCollection::new()
        .apply_config_at::<MyConfigOptions>(config, "MyOptions")
        .add(MyConfigValidation::transient())
        .build_provider()?;
    let options = provider.get_required::<dyn Options<MyConfigOptions>>();

    println!("Key1 = {}", &options.value().key1);
    Ok(())
}
```

Order of operation:

1. Register options services, including [OptionsFactory], via [apply_config_at]
2. Register `MyConfigValidation` as [ValidationOptions]
3. Enforce validation through
   1. [ServiceProvider::get_required], which calls
   2. [OptionsFactory], which calls
   3. `MyConfigValidation::validate`
   4. [Options::value] returns a valid `MyConfigOptions` or panics

>A panic is an unfortunate, current limitation of resolution from DI. For validation not to not panic, the injected
>service would need to be `Result<Ref<dyn Options<MyConfigOptions>>, _>`, which is possible, but not ergonomic.
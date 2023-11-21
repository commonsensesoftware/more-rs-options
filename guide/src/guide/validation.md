# Validation

Options validation enables configured option values to be validated. Validation is performed via `ValidationOptions<T>`, which is typically invoked during options construction through `OptionsFactory<T>` rather than imperatively.

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
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct MyConfigOptions {
    pub key1: String,
    pub key2: usize,
    pub key3: usize,
}
```

The following code:

- uses dependency injection (DI).
- calls `add_options` to get an `OptionsBuilder<T>` that binds to the `MyConfigOptions` struct.
- invokes a closure to validate the struct.

```rust
use config::{*, ext::*};
use di::*;
use options::ext::*;

fn main() {
    let config = Rc::from(
        DefaultConfigurationBuilder::new()
            .add_json_file("appsettings.json")
            .build()
            .unwrap()
            .as_config(),
    );
    let provider = ServiceCollection::new()
        .apply_config_at::<MyConfigOptions>(config, "MyConfig")
        .validate(
            |options| options.key2 == 0 || options.key3 > options.key2,
            "Key3 must be > than Key2.")
        .build_provider()
        .unwrap();
}
```

Dependency injection is not required to enforce validation, but it is the simplest and fastest way to compose all of the necessary pieces together.

## Implementing `ValidateOptions<T>`

`ValidateOptions<T>` enables moving the validation code out of a closure and into a struct. The following struct implements `ValidateOptions<T>`:

```rust
use options::*;

#[derive(Default)]
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
use config::{*, ext::*};
use di::*;
use options::{*, ext::*};

fn main() {
    let config = Rc::from(
        DefaultConfigurationBuilder::new()
            .add_json_file("appsettings.json")
            .build()
            .unwrap()
            .as_config(),
    );
    let provider = ServiceCollection::new()
        .apply_config_at::<MyConfigOptions>(config, "MyOptions")
        .add(transient::<dyn ValidateOptions<MyConfigOptions>, MyConfigValidation>()
             .from(|_| Rc::new(MyConfigValidation::default())))
        .build_provider()
        .unwrap();
    let options = provider.get_required::<dyn Options<MyConfigOptions>>();

    println!("Key1 = {}", &options.value().key1);
}
```

Order of operation:

1. Register options services, including `OptionsFactory<MyConfigOptions>`, via `apply_config_at`
2. Register `MyConfigValidation` as `ValidationOptions<MyConfigOptions>`
3. Enforce validation through
   1. `ServiceProvider.get_required`, which calls
   2. `OptionsFactory<MyConfigOptions>`, which calls
   3. `MyConfigValidation.validate`
   4. `Options<MyConfigOptions>::value` returns a valid `MyConfigOptions` or panics
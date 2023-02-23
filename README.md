# More Options Crate

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/more-options.svg
[crates-url]: https://crates.io/crates/more-options
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/commonsensesoftware/more-rs-options/blob/main/LICENSE

This library contains all of the fundamental abstractions for defining configuration options.

## Features

This crate provides the following features:

- _Default_ - Abstractions for the options
- **di** - Provides dependency injection extensions
- **cfg** - Provides dependency injection extensions to bind configurations to options
- **async** - Provides features for using options in an asynchronous context

-----

## Options Pattern

The options pattern uses classes to provide strongly typed access to groups of related settings. Options also provide a mechanism to validate configuration data. For more information, see the
[Options validation](#Options-Validation) section.

### Bind Hierarchical Configuration

The preferred way to read related configuration values is using the options pattern. For example, to read the
following configuration values:

```json
"Position": {
    "Title": "Editor",
    "Name": "Joe Smith"
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

- Must be public.
- Should implement the `Default` trait; otherwise a custom `OptionsFactory<TOptions>` is required.
- All public read-write fields of the type are bound.

The following code:

- Calls `ConfigurationBinder.bind` to bind the `PositionOptions` class to the `Position` section.
- Displays the `Position` configuration data.
- Requires the **binder** feature to be enabled
  - Which transitively enables the **serde** feature

```rust
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
        Self { config: config }
    }

    pub get(&self) -> String {
        let mut options = PositionOptions::default();
        let section = self.config.section("Position").bind(&mut options);
        format!("Title: {}\nName: {}", options.title, options.name)
    }
}
```

`ConfigurationBinder.reify<T>` binds and returns the specified type. `ConfigurationBinder.reify<T>` may be more convenient than using `ConfigurationBinder.bind`. The following code shows how to use `ConfigurationBinder.reify<T>` with the PositionOptions struct:

```rust
pub TestModel<'a> {
    config: &'a dyn Configuration
}

impl<'a> TestModel<'a> {
    pub new(config: &dyn Configuration) -> Self {
        Self { config: config }
    }

    pub get(&self) -> String {
        let options: PositionOptions = self.config.section("Position").reify();
        format!("Title: {}\nName: {}", options.title, options.name)
    }
}
```

An alternative approach when using the _**options pattern**_ is to bind the
`Position` section and add it to the dependency injection service container.
In the following code, `PositionOptions` is added to the service container with
`configure` and bound to configuration:

```rust
pub TestModel {
    options: Rc<dyn Options<Position>>
}

impl TestModel {
    pub new(options: Rc<dyn Options<Position>>) -> Self {
        Self { options: options }
    }

    pub get(&self) -> String {
        let value = self.options.value();
        format!("Title: {}\nName: {}", value.title, value.name)
    }
}

fn main() {
    let path = PathBuf::from("./appsettings.json");
    let config = Rc::from(
        DefaultConfigurationBuilder::new()
            .add_json_file(&path)
            .build()
            .as_config(),
    );
    let provider = ServiceCollection::new()
        .add(transient_as_self::<TestModel>())
        .apply_config::<PositionOptions>(config)
        .build_provider()
        .unwrap();
    let model = provider.get_required::<TestModel>();

    println!("{}", model.get())
}
```

## Options Traits

`Options<TOptions>`:
- Does **not** support:
    - Reading of configuration data after the app has started.
- Is registered as a `Singleton` and can be injected into any service lifetime.

`OptionsSnapshot<TOptions>`:
- Is useful in scenarios where options should be recomputed on every request.
- Is registered as `Scoped` and therefore can't be injected into a `Singleton` service.

`OptionsMonitor<TOptions>`:
- Is used to retrieve options and manage options notifications for `TOptions` instances.
- Is registered as a `Singleton` and can be injected into any service lifetime.
- Supports:
  - Change notifications
  - Reloadable configuration
  - Selective options invalidation (`OptionsMonitorCache<TOptions>`)

[Post-configuration](#options-post-configuration) scenarios enable setting or changing
options after all `ConfigureOptions<TOptions>` configuration occurs.

`OptionsFactory<TOptions>` is responsible for creating new options instances. It has a single
`create` method. The default implementation takes all registered `ConfigureOptions<TOptions>`
and `PostConfigureOptions<TOptions>` and runs all the configurations first, followed by the
post-configuration.

`OptionsMonitorCache<TOptions>` is used by `OptionsMonitor<TOptions>` to cache `TOptions` instances.
The `OptionsMonitorCache<TOptions>` invalidates options instances in the monitor so that the value
is recomputed (`try_remove`). Values can be manually introduced with `try_add`. The `clear` method
is used when all named instances should be recreated on demand.

## Use OptionsSnapshot to Read Updated Data

Using `OptionsSnapshot<TOptions>`:

- Options are computed once per request when accessed and cached for the lifetime of the request.
- May incur a significant performance penalty because it's a `Scoped` service and is recomputed per request.
- Changes to the configuration are read after the app starts when using configuration providers that support reading updated configuration values.

The difference between `OptionsMonitor<TOptions>` and `OptionsSnapshot<TOptions>` is that:

- `OptionsMonitor<TOptions>` is a `Singleton` service that retrieves current option values at any time, which is especially useful in singleton dependencies.
- `OptionsSnapshot<TOptions>` is a `Scoped` service and provides a snapshot of the options at the time the `OptionsSnapshot<TOptions>` object is constructed. Options snapshots are designed for use with transient and scoped dependencies.

The following code uses `OptionsSnapshot<TOptions>`:

```rust
pub TestSnapModel {
    snapshot: Rc<dyn OptionsSnapshot<MyOptions>>
}

impl TestSnapModel {
    pub new(snapshot: Rc<dyn OptionsSnapshot<MyOptions>>) -> Self {
        Self { snapshot: snapshot }
    }

    pub get(&self) -> String {
        let options = self.snapshot.get(None);
        format!("Option1: {}\nOption2: {}", options.option1, options.option2)
    }
}

fn main() {
    let path = PathBuf::from("./appsettings.json");
    let config = Rc::from(
        DefaultConfigurationBuilder::new()
            .add_json_file(&path)
            .build()
            .as_config(),
    );
    let provider = ServiceCollection::new()
        .add(transient_as_self::<TestSnapModel>())
        .apply_config_at::<MyOptions>(config, "MyOptions")
        .build_provider()
        .unwrap();
    let model = provider.get_required::<TestSnapModel>();

    println!("{}", model.get())
}
```

## OptionsMonitor

The following code registers a configuration instance which `MyOptions` binds against:

```rust
pub TestMonitorModel {
    monitor: Rc<dyn OptionsMonitor<MyOptions>>
}

impl TestMonitorModel {
    pub new(monitor: Rc<dyn OptionsMonitor<MyOptions>>) -> Self {
        Self { monitor: monitor }
    }

    pub get(&self) -> String {
        let options = self.monitor.get(None);
        format!("Option1: {}\nOption2: {}", options.option1, options.option2)
    }
}

fn main() {
    let path = PathBuf::from("./appsettings.json");
    let config = Rc::from(
        DefaultConfigurationBuilder::new()
            .add_json_file(&path)
            .build()
            .as_config(),
    );
    let provider = ServiceCollection::new()
        .add(transient_as_self::<TestMonitorModel>())
        .apply_config_at::<MyOptions>(config, "MyOptions")
        .build_provider()
        .unwrap();
    let model = provider.get_required::<TestMonitorModel>();

    println!("{}", model.get())
}
```

## Use Dependency Injection to Configure Options

Services can be accessed from dependency injection while configuring options in two ways:

- Pass a configuration function
```rust
services.add_options::<MyOptions>()
        .configure(|options| options.count = 1);
services.configure_options::<MyAltOptions>(|options| options.count = 1);
services.add_named_options::<MyOtherOptions>("name")
        .configure5(
            |options,
            s2: Rc<Service2>,
            s1: Rc<Service1>,
            s3: Rc<Service3>,
            s4: Rc<Service4>
            s4: Rc<Service5>| {
                options.property = do_something_with(s1, s2, s3, s4, s5);
            });
```
- Implement the `ConfigureOptions<TOptions>` trait and register it as a service

It is recommended to pass a configuration closure to one of the `configure` functions
since creating a struct is more complex. Creating a struct is equivalent to what the
framework does when calling any of the `configure` functions. Calling one of the
`configure` functions registers a transient `ConfigureOptions<TOptions>`, which
initializes with the specified service types.

| Function     | Description |
| ------------ | ----------- |
| `configure`  | Configures the options without using any services |
| `configure1` | Configures the options using a single dependency |
| `configure2` | Configures the options using 2 dependencies |
| `configure3` | Configures the options using 3 dependencies |
| `configure4` | Configures the options using 4 dependencies |
| `configure5` | Configures the options using 5 dependencies |
| `post_configure`  | Post-configures the options without using any services |
| `post_configure1` | Post-configures the options using a single dependency |
| `post_configure2` | Post-configures the options using 2 dependencies |
| `post_configure3` | Post-configures the options using 3 dependencies |
| `post_configure4` | Post-configures the options using 4 dependencies |
| `post_configure5` | Post-configures the options using 5 dependencies |
| `validate`  | Validates the options without using any services |
| `validate1` | Validates the options using a single dependency |
| `validate2` | Validates the options using 2 dependencies |
| `validate3` | Validates the options using 3 dependencies |
| `validate4` | Validates the options using 4 dependencies |
| `validate5` | Validates the options using 5 dependencies |

## Options Validation

Options validation enables option values to be validated.

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

The following code:

- Calls `add_options` to get an `OptionsBuilder<TOptions>` that binds to the
  `MyConfigOptions` struct.
- Invokes a closure to validate the struct.

```rust
fn main() {
    let path = PathBuf::from("./appsettings.json");
    let config = Rc::from(
        DefaultConfigurationBuilder::new()
            .add_json_file(&path)
            .build()
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

It is recommended to validate options via a closure as opposed to implementing
`ValidateOptions<TOptions>` directly. The default validation consumers, such as
`OptionsFactory<TOptions>`, panic if there are any validation errors as the
application is considered to be in an invalid state.

### `ValidateOptions<TOptions>`

The following struct implements `ValidateOptions<TOptions>`:

```rust
#[derive(Default)]
struct MyConfigValidation;

impl ValidationOptions<MyConfigOptions> for MyConfigValidation {
    fn valid(&self, name: Option<&str>, options: &MyConfigOptions) -> ValidateOptionsResult {
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

`ValidateOptions` enables moving the validation code out of a closure and into a struct.

Using the preceding code, validation is enabled with the following code:

```rust
fn main() {
    let path = PathBuf::from("./appsettings.json");
    let config = Rc::from(
        DefaultConfigurationBuilder::new()
            .add_json_file(&path)
            .build()
            .as_config(),
    );
    let provider = ServiceCollection::new()
        .apply_config_at::<MyOptions>(config, "MyOptions")
        .add(
            transient::<dyn ValidateOptions<MyConfigOptions>, MyConfigValidation>()
            .from(|_| Rc::new(MyConfigValidation::default())))
        .build_provider()
        .unwrap();
    let options = provider.get_required::<MyConfigOptions>();
}
```

## Options Post-Configuration

Set post-configuration with `PostConfigureOptions<TOptions>`. Post-configuration
runs after all `ConfigureOptions<TOptions>` configuration occurs:

```rust
fn main() {
    let provider = ServiceCollection::new()
        .post_configure_options::<TestOptions>(|options| options.enabled = true)
        .build_provider()
        .unwrap();
}
```

`post_configure_options` applies to all instances. To apply a named configuration use
`post_configure_named_options`.

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/commonsensesoftware/more-rs-options/blob/main/LICENSE
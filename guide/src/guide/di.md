# Dependency Injection

An alternative approach when using the options pattern is to bind an entire configuration or section of it through a dependency injection (DI) service container. Dependency injection extensions are provided by the [more-di](https://crates.io/crates/more-di) crate.

In the following code, `PositionOptions` is added to the service container with `apply_config` and bound to loaded configuration:

```rust
use config::{*, ext::*};
use options::{Options, ext::*};
use serde::Deserialize;
use std::rc::Rc;

#[derive(Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct PositionOptions {
    pub title: String,
    pub name: String,
}

pub TestModel {
    options: Rc<dyn Options<Position>>
}

impl TestModel {
    pub new(options: Rc<dyn Options<Position>>) -> Self {
        Self { options }
    }

    pub get(&self) -> String {
        let value = self.options.value();
        format!("Title: {}\nName: {}", value.title, value.name)
    }
}

fn main() {
    let config = Rc::from(
        DefaultConfigurationBuilder::new()
            .add_json_file("appsettings.json")
            .build()
            .unwrap()
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

## Options Configuration

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
- Implement the `ConfigureOptions<T>` trait and register it as a service

It is recommended to pass a configuration closure to one of the `configure` functions since creating a struct is more complex. Creating a struct is equivalent to what the framework does when calling any of the `configure` functions. Calling one of the `configure` functions registers a transient `ConfigureOptions<T>`, which initializes with the specified service types.

| Function     | Description                                       |
| ------------ | ------------------------------------------------- |
| `configure`  | Configures the options without using any services |
| `configure1` | Configures the options using a single dependency  |
| `configure2` | Configures the options using 2 dependencies       |
| `configure3` | Configures the options using 3 dependencies       |
| `configure4` | Configures the options using 4 dependencies       |
| `configure5` | Configures the options using 5 dependencies       |


## Options Post-Configuration

Set post-configuration with `PostConfigureOptions<T>`. Post-configuration runs after all `ConfigureOptions<T>` configuration occurs.

Services can be accessed from dependency injection while configuring options in two ways:

- Pass a configuration function
```rust
services.add_options::<MyOptions>()
        .post_configure(|options| options.count = 1);
services.post_configure_options::<MyAltOptions>(|options| options.count = 1);
services.add_named_options::<MyOtherOptions>("name")
        .post_configure5(
            |options,
            s2: Rc<Service2>,
            s1: Rc<Service1>,
            s3: Rc<Service3>,
            s4: Rc<Service4>
            s4: Rc<Service5>| {
                options.property = do_something_with(s1, s2, s3, s4, s5);
            });
```
- Implement the `PostConfigureOptions<T>` trait and register it as a service

`post_configure_options` applies to all instances. To apply a named configuration use `post_configure_named_options`. It is recommended to pass a configuration closure to one of the `post_configure` functions since creating a struct is more complex. Creating a struct is equivalent to what the framework does when calling any of the `post_configure` functions. Calling one of the `post_configure` functions registers a transient `PostConfigureOptions<T>`, which initializes with the specified service types.

| Function          | Description                                            |
| ----------------- | ------------------------------------------------------ |
| `post_configure`  | Post-configures the options without using any services |
| `post_configure1` | Post-configures the options using a single dependency  |
| `post_configure2` | Post-configures the options using 2 dependencies       |
| `post_configure3` | Post-configures the options using 3 dependencies       |
| `post_configure4` | Post-configures the options using 4 dependencies       |
| `post_configure5` | Post-configures the options using 5 dependencies       |

## Options Validation

Validation is performed with `ValidateOptions<T>`. Validation runs after all `ConfigureOptions<T>` and `PostConfigureOptions<T>` occurs.

Services can be accessed from dependency injection while validating options in two ways:

- Pass a validation function
```rust
services.add_options::<MyOptions>()
        .configure(|options| options.count = 1)
        .validate(|options| options.count > 0, "Count must be greater than 0.");
services.add_named_options::<MyOtherOptions>("name")
        .configure(|options| options.count = 1)
        .validate5(
            |options,
            s2: Rc<Service2>,
            s1: Rc<Service1>,
            s3: Rc<Service3>,
            s4: Rc<Service4>
            s4: Rc<Service5>| do_complex_validation(s1, s2, s3, s4, s5));
```
- Implement the `ValidateOptions<T>` trait and register it as a service

It is recommended to pass a validation closure to one of the `validate` functions since creating a struct is more complex. Creating a struct is equivalent to what the framework does when calling any of the `validate` functions. Calling one of the `validate` functions registers a transient `ValidateOptions<T>`, which initializes with the specified service types.

| Function    | Description                                      |
| ----------- | ------------------------------------------------ |
| `validate`  | Validates the options without using any services |
| `validate1` | Validates the options using a single dependency  |
| `validate2` | Validates the options using 2 dependencies       |
| `validate3` | Validates the options using 3 dependencies       |
| `validate4` | Validates the options using 4 dependencies       |
| `validate5` | Validates the options using 5 dependencies       |
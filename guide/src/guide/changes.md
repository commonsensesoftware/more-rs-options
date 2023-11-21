# Runtime Changes

The options framework supports responding to setting changes at runtime when they occur. There are a number of scenarios when that may happen, such as an underlying configuration file has changed. The options framework doesn't understand how or what has changed, only that a change has occurred. In response to the change, the corresponding options will be updated.

## Snapshot

When using `OptionsSnapshot<T>`:

- options are computed once per request when accessed and cached for the lifetime of the request.
- may incur a significant performance penalty because it's a `Scoped` service and is recomputed per request.
- changes to the configuration are read after the application starts when using configuration providers that support reading updated configuration values.

The following code uses `OptionsSnapshot<T>`:

```rust
use crate::*;
use config::{*, ext::*};
use options::{OptionsSnapshot, ext::*};
use std::rc::Rc;

pub TestSnapModel {
    snapshot: Rc<dyn OptionsSnapshot<MyOptions>>
}

impl TestSnapModel {
    pub new(snapshot: Rc<dyn OptionsSnapshot<MyOptions>>) -> Self {
        Self { snapshot }
    }

    pub get(&self) -> String {
        let options = self.snapshot.get(None);
        format!("Option1: {}\nOption2: {}", options.option1, options.option2)
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
        .add(transient_as_self::<TestSnapModel>())
        .apply_config_at::<MyOptions>(config, "MyOptions")
        .build_provider()
        .unwrap();
    let model = provider.get_required::<TestSnapModel>();

    println!("{}", model.get())
}
```

## Monitor

Monitored options will reflect the current setting values whenever an underlying source changes.

The difference between `OptionsMonitor<T>` and `OptionsSnapshot<T>` is that:

- `OptionsMonitor<T>` is a `Singleton` service that retrieves current option values at any time, which is especially useful in singleton dependencies.
- `OptionsSnapshot<T>` is a `Scoped` service and provides a snapshot of the options at the time the `OptionsSnapshot<T>` struct is constructed. Options snapshots are designed for use with `Transient` and `Scoped` dependencies.

The following code registers a configuration instance which `MyOptions` binds against:

```rust
use crate::*;
use config::{*, ext::*};
use options::{OptionsMonitor, ext::*};
use std::rc::Rc;

pub TestMonitorModel {
    monitor: Rc<dyn OptionsMonitor<MyOptions>>
}

impl TestMonitorModel {
    pub new(monitor: Rc<dyn OptionsMonitor<MyOptions>>) -> Self {
        Self { monitor }
    }

    pub get(&self) -> String {
        let options = self.monitor.get(None);
        format!("Option1: {}\nOption2: {}", options.option1, options.option2)
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
        .add(transient_as_self::<TestMonitorModel>())
        .apply_config_at::<MyOptions>(config, "MyOptions")
        .build_provider()
        .unwrap();
    let model = provider.get_required::<TestMonitorModel>();

    println!("{}", model.get())
}
```
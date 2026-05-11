{{#include links.md}}

# Runtime Changes

The options framework supports responding to setting changes at runtime when they occur. There are a number of scenarios
when that may happen, such as an underlying configuration file has changed. The options framework doesn't understand how
or what has changed, only that a change has occurred. In response to the change, the corresponding options will be
updated.

## Options Snapshot

When using an options [Snapshot]:

- options are computed once per request when accessed and cached for the lifetime of the request.
- may incur a significant performance penalty because it's a [Scoped] service and is recomputed per request.
- changes to the configuration are read after the application starts when using configuration providers that support reading updated configuration values.

The following code uses an options [Snapshot]:

```rust
use crate::MyOptions;
use config::prelude::*;
use di::{injectable, Injectable, ServiceCollection};
use options::{prelude::*, Snapshot, Ref};
use std::error::Error;

pub struct TestSnapModel {
    snapshot: Ref<dyn Snapshot<MyOptions>>
}

#[injectable]
impl TestSnapModel {
    pub fn new(snapshot: Ref<dyn Snapshot<MyOptions>>) -> Self {
        Self { snapshot }
    }

    pub fn get(&self) -> String {
        let options = self.snapshot.get_unchecked();
        format!("Option1: {}\nOption2: {}", options.option1, options.option2)
    }
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let config = config::builder().add_json_file("appsettings.json").build()?;
    let provider = ServiceCollection::new()
        .add(TestSnapModel::transient())
        .apply_config_at::<MyOptions>(config, "MyOptions")
        .build_provider()?;
    let model = provider.get_required::<TestSnapModel>();

    println!("{}", model.get());
    Ok(())
}
```

## Options Monitor

Monitored options will reflect the current setting values whenever an underlying source changes.

The difference between an options [Monitor] and [Snapshot] are that:

- [Monitor] is a [Singleton] service that retrieves current option values at any time, which is especially useful in singleton dependencies.
- [Snapshot] is a [Scoped] service and provides a snapshot of the options at the time the [Snapshot] is constructed. Options snapshots are designed for use with [Transient] and [Scoped] dependencies.

The following code registers a configuration instance which `MyOptions` binds against:

```rust
use crate::MyOptions;
use config::{prelude::*, ReloadableConfiguration};
use di::{injectable, Injectable, ServiceCollection};
use options::{prelude::*, Monitor, Ref};
use std::convert::TryInto;
use std::error::Error;

pub TestMonitorModel {
    monitor: Ref<dyn Monitor<MyOptions>>
}

#[injectable]
impl TestMonitorModel {
    pub new(monitor: Ref<dyn Monitor<MyOptions>>) -> Self {
        Self { monitor }
    }

    pub get(&self) -> String {
        let options = self.monitor.get_unchecked();
        format!("Option1: {}\nOption2: {}", options.option1, options.option2)
    }
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let config: ReloadableConfiguration = config::builder()
        .add_json_file("appsettings.json".is().reloadable())
        .try_into()?;
    let provider = ServiceCollection::new()
        .add(TestMonitorModel::transient())
        .apply_config_at::<MyOptions>(config, "MyOptions")
        .build_provider()?;
    let model = provider.get_required::<TestMonitorModel>();

    println!("{}", model.get());
    Ok(())
}
```

>In order to detect and bind configuration changes, a configuration `Builder` needs to be converted into a
>`ReloadableConfiguration` instead of building built a `Configuration`.
{{#include links.md}}

# Abstractions

The options framework contains a common set traits and behaviors for numerous scenarios.

[Ref] is a type alias depending on which features are enabled:

- _default_: `options::Ref` → `std::rc::Rc`
- **async**: `options::Ref` → `std::sync::Arc`
- **async** + **di**: `options::Ref` → `di::Ref`

## Options

Any Rust `struct` that represents configurable options.

- Does **not** support:
    - Reading of configuration data after the application has started.
- Is registered as a [Singleton] and can be injected into any service lifetime when using dependency injection.

## Options Snapshot

```rust
pub trait Snapshot<T> {
    fn get(&self) -> Result<Ref<T>, validation::Error>;
    fn get_unchecked(&self) -> Ref<T>;
    fn get_named(&self, name: &str) -> Result<Ref<T>, validation::Error>;
    fn get_named_unchecked(&self, name: &str) -> Ref<T>;
}
```

- Is useful in scenarios where options should be recomputed on every request.
- Is registered as [Scoped] and therefore can't be injected into a [Singleton] service when using dependency injection.

## Options Monitor

```rust
pub trait Monitor<T> {
    fn get(&self) -> Result<Ref<T>, validation::Error>;
    fn get_unchecked(&self) -> Ref<T>;
    fn get_named(&self, name: &str) -> Result<Ref<T>, validation::Error>;
    fn get_named_unchecked(&self, name: &str) -> Ref<T>;
    fn on_change(
        &self,
        callback: Box<dyn Fn(&str, Ref<T>) + Send + Sync>) -> Subscription<T>;
}
```

- Is used to retrieve options and manage options notifications for `T` instances.
- Is registered as a [Singleton] and can be injected into any service lifetime when using dependency injection.
- Supports:
  - Change notifications
  - Reloadable configuration
  - Selective options invalidation ([MonitorCache])

## Options Monitor Cache

```rust
pub trait MonitorCache<T> {
    fn get_or_add(
        &self,
        name: &str,
        create: &dyn Fn(&str) -> Result<T, validation::Error>)
        -> Result<Ref<T>, validation::Error>;
    fn try_add(&self, name: &str, options: T) -> bool;
    fn try_remove(&self, name: &str) -> bool;
    fn clear(&self);
}
```

- A cache of `T` instances.
- Handles invaliding monitored instances when underlying changes occur.

## Configure Options

```rust
pub trait Configure<T> {
    fn run(&self, name: &str, options: &mut T);
}
```

- Configures options when they are being instantiated.
- Can be implemented directly or maps to compatible closures.

## Post-Configure Options

```rust
pub trait PostConfigure<T> {
    fn run(&self, name: &str, options: &mut T);
}
```

- Configures options after they have been instantiated.
- Can be implemented directly or maps to compatible closures.
- Enable setting or changing options after all [Configure] operations occur.

## Validate Options

Validation is part of the `validation` module.

```rust
pub trait Validate<T> {
    fn run(&self, name: &str, options: &T) -> Result;
}
```

- Validates options after they have been instantiated and configured.
- Can be implemented directly or maps to compatible closures.

## Options Factory

```rust
pub trait Factory<T> {
    fn create(&self, name: &str) -> Result<T, validation::Error>;
}
```

- Responsible for creating new options instances.
- The default implementation run all configured instance of:
  - `Configure`
  - `PostConfigure`
  - `Validate`

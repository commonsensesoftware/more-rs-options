# Abstractions

The options framework contains a common set traits and behaviors for numerous scenarios.

`Ref<T>` is a type alias depending on which features are enabled:

- _default_: `options::Ref<T>` → `std::rc::Rc<T>`
- **async**: `options::Ref<T>` → `std::sync::Arc<T>`
- **async** + **di**: `options::Ref<T>` → `di::Ref<T>`

## Options

```rust
pub trait Options<T> {
    fn value(&self) -> Ref<T>;
}
```

- Does **not** support:
    - Reading of configuration data after the application has started.
- Is registered as a `Singleton` and can be injected into any service lifetime when using dependency injection.

## Options Snapshot

```rust
pub trait OptionsSnapshot<T> {
    fn get(&self, name: Option<&str>) -> Ref<T>;
}
```

- Is useful in scenarios where options should be recomputed on every request.
- Is registered as `Scoped` and therefore can't be injected into a `Singleton` service when using dependency injection.

## Options Monitor

```rust
pub trait OptionsMonitor<T> {
    fn current_value(&self) -> Ref<T>;
    fn get(&self, name: Option<&str>) -> Ref<T>;
    fn on_change(
        &self,
        listener: Box<dyn Fn(Option<&str>, Ref<T>) + Send + Sync>) -> Subscription<T>;
}
```

- Is used to retrieve options and manage options notifications for `T` instances.
- Is registered as a `Singleton` and can be injected into any service lifetime when using dependency injection.
- Supports:
  - Change notifications
  - Reloadable configuration
  - Selective options invalidation (`OptionsMonitorCache<T>`)

## Options Monitor Cache

```rust
pub trait OptionsMonitorCache<T> {
    fn get_or_add(
        &self,
        name: Option<&str>,
        create_options: &dyn Fn(Option<&str>) -> T) -> Ref<T>;
    fn try_add(&self, name: Option<&str>, options: T) -> bool;
    fn try_remove(&self, name: Option<&str>) -> bool;
    fn clear(&self);
}
```

- A cache of `T` instances.
- Handles invaliding monitored instances when underlying changes occur.

## Configure Options

```rust
pub trait ConfigureOptions<T> {
    fn configure(&self, name: Option<&str>, options: &mut T);
}
```

- Configures options when they are being instantiated.

## Post-Configure Options

```rust
pub trait PostConfigureOptions<T> {
    fn post_configure(&self, name: Option<&str>, options: &mut T);
}
```

- Configures options after they have been instantiated.
- Enable setting or changing options after all `ConfigureOptions<TOptions>` configuration occurs.

## Validate Options

```rust
pub trait ValidateOptions<T> {
    fn validate(&self, name: Option<&str>, options: &T) -> ValidateOptionsResult;
}
```

- Validates options after they have been instantiated and configured.

## Options Factory

```rust
pub trait OptionsFactory<T> {
    fn create(&self, name: Option<&str>) -> Result<T, ValidateOptionsResult>;
}
```

- Responsible for creating new options instances.
- The default implementation run all configured instance of:
  - `ConfigureOptions<TOptions>`
  - `PostConfigureOptions<TOptions>`
  - `ValidateOptions<TOptions>`

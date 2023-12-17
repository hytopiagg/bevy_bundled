# `bevy_bundled`
A small derive macro for turning structs into bundles of components and resources without creating types for each field

## What this does
This crate creates a `Bundled` derive macro, which creates a mirror of your struct in which each field is a component, and the mirror is a bundle. Additionally this crate introduces a `ResourceBundle` abstraction, in which individual struct fields can be accessed seperately as `Resource`s.

## Minimal example
### `Bundled`
```rust
#[derive(Default, Bundled)]
struct Player {
    health: f32,
    position: Vec3,
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Player::default().bundled());
}

fn health_system(health: Query<&Player::Health>) {
    // ...
}
```

### `ResourceBundle`
```rust
#[derive(Default, ResourceBundle)]
struct Player {
    health: f32,
    position: Vec3,
}

fn setup_system(mut commands: Commands) {
    commands.init_resource_bundle::<Player>();
}

fn health_system(health: Res<Player::Health>) {
    // ...
}
```

### 

## To do
* [ ] Accessibility
    * [x] Documentation
    * [ ] Better Errors
    * [ ] Better Examples
        * [ ] `ResourceBundle` Example
        * [ ] Idiomatic example for `Bundled`
* [ ] Unnamed/Tuple Structs
* [x] Marker Components
    * Possible `#[marked]` attribute which creates a simple `Marker` type and field
        * Should automatically be added when `.into` is called from base struct
    * Marked is the default, with a `#[marked]` attribute to make it explicit, and an `#[unmarked]` to disable marking
* [x] Bundled Resources
    * [ ] Mixed `Resource`s and `NonSend`
* [ ] Possibly generate `SystemParam`s for `ResourceBundle`s and `Bundled`s to access whole struct easily
* [ ] Unwrapped Fields
    * [ ] Nested Bundles
* [ ] Refactoring
    * [ ] Use `quote_spanned` to ensure hygene

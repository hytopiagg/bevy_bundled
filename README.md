# `bevy_bundled`
A small derive macro for turning structs into bundles without creating types for each field

## What this does
This crate creates a `Bundled` derive macro, which creates a mirror of your struct in which each field is a component, and the mirror is a bundle

## Minimal example
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

## To do
* [ ] Accessibility
    * [x] Documentation
    * [ ] Better Errors
    * [x] Better Examples
* [ ] Unnamed/Tuple Structs
* [x] Marker Components
    * Possible `#[marked]` attribute which creates a simple `Marker` type and field
        * Should automatically be added when `.into` is called from base struct
    * Marked is the default, with a `#[marked]` attribute to make it explicit, and an `#[unmarked]` to disable marking
* [ ] Bundled Resources
* [ ] Unwrapped Fields
    * [ ] Nested Bundles
* [ ] Refactoring
    * [ ] Use `quote_spanned` to ensure hygene

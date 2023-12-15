# `bevy_bundled`
## A small derive macro for turning large structs into bundles without creating types for each field

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

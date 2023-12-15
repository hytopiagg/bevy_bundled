# `bevy_bundled`
## A small derive macro for turning large structs into bundles without creating types for each field

## To do
* [ ] Accessibility
    * [ ] Documentation
    * [ ] Better Errors
* [ ] Better Examples
* [ ] Unnamed/Tuple Structs
* [ ] Marker Components
    * Possible `#[marked]` attribute which creates a simple `Marker` type and field
        * Should automatically be added when `.into` is called from base struct
* [ ] Bundled Resources
* [ ] Refactoring
    * [ ] Use `quote_spanned` to ensure hygene

## Limitations
* You can't use `impl`s on `Bundled` types
    * This is because in order to make the field types easily available we need to create an `impl` block
    * The original plan was a sealed trait containing the types as associated types
        * Because of Rust issue [#104119](https://github.com/rust-lang/rust/issues/104119), doing so would require a fully qualified type to access those field types
        * Easy access to field types was deemed more important than `impl` blocks on `Bundled` types
            * We're open to other API designs which allow ease of access to these types without breaking `impl` blocks

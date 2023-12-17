use bevy::prelude::App;
use heck::{AsPascalCase, AsSnakeCase};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Fields, FieldsNamed, Item};

/// # `Bundled` Derive Macro
///
/// This macro facilitates using struct fields as seperate components with bundles without declaring newtypes for
/// each field, and initializing each when declaring the bundle
///
/// ## Usage
/// To use spawn `Bundled` struct with Bevy, initialize it like normal, then call `Self::bundled` to
/// turn it into a bundle
///
/// To get the type of the `Bundle` associated with the struct, use the generated `Self::Bundled`
/// type
///
/// To query for a field, query for the generated type corrosponding to the field. For example, a
/// field `health`, would have a corrosponding `Self::Health`. Additionally, unless unmarked (see
/// attributes), the struct will have a generated type `Self::Marker`, corrosponding to a marker
/// component generated and initialized for the struct.
///
/// ### Attributes
///   * `#[marked]`: Struct level attribute indicating a struct should have an additional generated marker
///   component for easy
///   querying. This behavior is enabled by default, so this attribute does nothing but make it
///   explicit.
///  * `#[unmarked]`: Struct level attribute indicating preventing the generation of a marker field and component.
/// *Note: This derive macro will panic if a struct is given both the `#[marked]` and `#[unmarked]`
/// attributes*
///
/// ## Examples
/// ```
/// use bevy::prelude::*;
/// use bevy_bundled::Bundled;
///
/// #[derive(Bundled)]
/// pub struct Player {
///     name: String,
///     health: i32,
/// }
///
/// fn main() {
///     let player = Player {
///         name: "Alice".to_string(),
///         health: 100,
///     };
///
///     let mut world = World::new();
///     world.spawn().insert_bundle(player.bundled());
/// }
/// ```
#[proc_macro_derive(Bundled, attributes(marked, unmarked))]
pub fn bundle(input: TokenStream) -> TokenStream {
    let item: Item = parse_macro_input!(input);
    match item {
        Item::Struct(ref st) => {
            let ident = &st.ident;
            let attrs = &st.attrs.iter().map(|x| x.meta.clone());

            let has_marked_attr = !attrs
                .clone()
                .filter(|x| match x.clone() {
                    syn::Meta::Path(m) => m
                        .get_ident()
                        .map(|ident| ident.to_string() == *"marked")
                        .unwrap_or(false),
                    _ => false,
                })
                .collect::<Vec<_>>()
                .is_empty();

            let has_unmarked_attr = !attrs
                .clone()
                .filter(|x| match x.clone() {
                    syn::Meta::Path(m) => m
                        .get_ident()
                        .map(|ident| ident.to_string() == *"unmarked")
                        .unwrap_or(false),
                    _ => false,
                })
                .collect::<Vec<_>>()
                .is_empty();

            if has_marked_attr && has_unmarked_attr {
                panic!("Cannot mark single struct as marked and unmarked");
            }

            let marked = !has_unmarked_attr;

            // Identifier for generated internal module
            let mod_ident = format_ident!("_{}", AsSnakeCase(st.ident.to_string()).to_string());

            // Identifier for component type (effectively type-level map)
            let component_ident = format_ident!("{}FieldComponent", st.ident);

            // Identifier for generated bundle struct
            let bundle_ident = format_ident!("{}Bundle", st.ident);

            let marker_ident = format_ident!("{}Marker", st.ident);

            let marker_declaration = if marked {
                Some(quote! {
                    #[derive(bevy::ecs::component::Component)]
                    pub struct #marker_ident;
                })
            } else {
                None
            };

            let marker_alias = if marked {
                Some(quote! {
                    pub type Marker = #mod_ident::#marker_ident;
                })
            } else {
                None
            };

            let marker_field = if marked {
                Some(quote! {
                    marker: #marker_ident,
                })
            } else {
                None
            };

            let marker_from = if marked {
                Some(quote! {
                    marker: #marker_ident,
                })
            } else {
                None
            };
            match st.fields {
                Fields::Named(FieldsNamed {
                    brace_token: _,
                    ref named,
                }) => {
                    let named = named.into_iter();

                    let fields_trait_impl = named.clone().enumerate().map(|(i, x)| {
                        let field_ident = Ident::new(
                            &AsPascalCase(x.ident.clone().unwrap().to_string()).to_string(),
                            x.ident.clone().unwrap().span(),
                        );
                        let field_ty = &x.ty;
                        quote! {
                        pub type #field_ident =
                            #mod_ident::#component_ident<#i, #field_ty>;
                        }
                    });

                    let bundle_inner_ident = named.clone().enumerate().map(|(i, x)| {
                        let field_ident = x.ident.clone();
                        let field_ty = &x.ty;
                        quote! {
                            #field_ident: #component_ident<#i, #field_ty>,
                        }
                    });

                    let from_inner = named.clone().enumerate().map(|(i, x)| {
                        let field_ident = x.ident.clone();
                        quote! {
                            #field_ident: #component_ident::<#i, _>(item.#field_ident),
                        }
                    });

                    quote! {

                    #[automatically_derived]
                    impl #ident {
                        #(#fields_trait_impl)*
                        pub type Bundled = #mod_ident::#bundle_ident;
                        #marker_alias

                        pub fn bundled(self) -> Self::Bundled {
                            self.into()
                        }
                    }


                    pub mod #mod_ident {
                        use super::*;
                        #marker_declaration

                        #[derive(bevy::ecs::bundle::Bundle)]
                        pub struct #bundle_ident {
                            #(#bundle_inner_ident)*
                            #marker_field
                        }

                        #[automatically_derived]
                        impl From<super::#ident> for #bundle_ident {
                            fn from(item: super::#ident) -> #bundle_ident {
                                #bundle_ident {
                                    #(#from_inner)*
                                    #marker_from
                                }
                            }

                        }

                        #[derive(bevy::prelude::Deref, bevy::prelude::DerefMut, bevy::ecs::component::Component)]
                        pub struct #component_ident<const FIELD: usize, T>(pub(super) T);
                    }
                    }
                    .into()
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

/// # `ResourceBundle` Derive Macro
///
/// This macro facilitates using creating a struct in which each field is a seperate resources (utilizing an abstraction called `ResourceBundle`s) without declaring newtypes for and manually initializing each individual resource.
///
/// ## Usage
/// To initialize a `ResourceBundle` struct with Bevy, use the `AppExtension` trait's
/// `insert_resource_bundle` function, which works analogously to `insert_resource`.
///
/// Additionally, for `ResourceBundle`s implementing `Default`,
/// you can use the `init_resource_bundle` function, which works analogously to `init_resource`.
///
/// To access a field as a `Resource`, query for the generated type corrosponding to the field. For example, a
/// field `health`, would have a corrosponding `Self::Health`, which can be accessed with a normal
/// `Res` or `ResMut` query.
///
/// ## Examples
/// ```
/// use bevy::prelude::*;
/// use bevy_bundled::{ResourceBundle, AppExtension};
///
/// #[derive(ResourceBundle)]
/// pub struct Player {
///     name: String,
///     health: i32,
/// }
///
/// fn main() {
///     let player = Player {
///         name: "Alice".to_string(),
///         health: 100,
///     };
///
///     App::new().insert_resource_bundle(player).run();
/// }
/// ```

#[proc_macro_derive(ResourceBundle)]
pub fn resource_bundle(input: TokenStream) -> TokenStream {
    let item: Item = parse_macro_input!(input);
    match item {
        Item::Struct(ref st) => {
            let ident = &st.ident;
            let attrs = &st.attrs.iter().map(|x| x.meta.clone());

            let has_marked_attr = !attrs
                .clone()
                .filter(|x| match x.clone() {
                    syn::Meta::Path(m) => m
                        .get_ident()
                        .map(|ident| ident.to_string() == *"marked")
                        .unwrap_or(false),
                    _ => false,
                })
                .collect::<Vec<_>>()
                .is_empty();

            let has_unmarked_attr = !attrs
                .clone()
                .filter(|x| match x.clone() {
                    syn::Meta::Path(m) => m
                        .get_ident()
                        .map(|ident| ident.to_string() == *"unmarked")
                        .unwrap_or(false),
                    _ => false,
                })
                .collect::<Vec<_>>()
                .is_empty();

            if has_marked_attr && has_unmarked_attr {
                panic!("Cannot mark single struct as marked and unmarked");
            }

            // Identifier for generated internal module
            let mod_ident = format_ident!("_{}", AsSnakeCase(st.ident.to_string()).to_string());

            // Identifier for component type (effectively type-level map)
            let component_ident = format_ident!("{}FieldResource", st.ident);

            match st.fields {
                Fields::Named(FieldsNamed {
                    brace_token: _,
                    ref named,
                }) => {
                    let named = named.into_iter();

                    let fields_trait_impl = named.clone().enumerate().map(|(i, x)| {
                        let field_ident = Ident::new(
                            &AsPascalCase(x.ident.clone().unwrap().to_string()).to_string(),
                            x.ident.clone().unwrap().span(),
                        );
                        let field_ty = &x.ty;
                        quote! {
                        pub type #field_ident =
                            #mod_ident::#component_ident<#i, #field_ty>;
                        }
                    });

                    let insert_self_inner = named.clone().enumerate().map(|(i, x)| {
                        let field_ident = x.ident.clone();
                        quote! {
                            .insert_resource(#mod_ident::#component_ident::<#i, _>(self.#field_ident))
                        }
                    });

                    let insert_self_commands_inner = named.clone().enumerate().map(|(i, x)| {
                        let field_ident = x.ident.clone();
                        quote! {
                            commands.insert_resource(#mod_ident::#component_ident::<#i, _>(self.#field_ident));
                        }
                    });

                    quote! {

                    #[automatically_derived]
                    impl #ident {
                        #(#fields_trait_impl)*
                    }

                    #[automatically_derived]
                    impl bevy_bundled::ResourceBundle for #ident {
                        fn insert_self_app(&self, app: &mut bevy::prelude::App) {
                            app
                                #(#insert_self_inner)*;
                        }

                        fn insert_self_commands(&self, commands: &mut bevy::prelude::Commands) {
                            #(#insert_self_commands_inner)*
                        }
                    }

                    pub mod #mod_ident {
                        use super::*;

                        #[derive(bevy::prelude::Deref, bevy::prelude::DerefMut, bevy::ecs::prelude::Resource)]
                        pub struct #component_ident<const FIELD: usize, T>(pub(super) T);
                    }
                    }
                    .into()
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

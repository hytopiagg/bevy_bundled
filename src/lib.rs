use heck::{AsPascalCase, AsSnakeCase};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Fields, FieldsNamed, Item};

#[proc_macro_derive(Bundled)]
pub fn bundle(input: TokenStream) -> TokenStream {
    let item: Item = parse_macro_input!(input);
    match item {
        Item::Struct(ref st) => {
            let ident = &st.ident;

            // Identifier for generated internal module
            let mod_ident = Ident::new(
                &format!("_{}", AsSnakeCase(st.ident.to_string())),
                st.ident.span(),
            );

            // Identifier for component type (effectively type-level map)
            let component_ident =
                Ident::new(&format!("{}FieldComponent", st.ident), st.ident.span());

            // Identifier for generated bundle struct
            let bundle_ident = Ident::new(&format!("{}Bundle", st.ident), st.ident.span());

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
                        type #field_ident =
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
                    }


                    pub mod #mod_ident {
                        #[derive(bevy::ecs::bundle::Bundle)]
                        pub struct #bundle_ident {
                            #(#bundle_inner_ident)*
                        }

                        #[automatically_derived]
                        impl From<super::#ident> for #bundle_ident {
                            fn from(item: super::#ident) -> #bundle_ident {
                                #bundle_ident {
                                    #(#from_inner)*
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

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

            // Identifier for private trait for sealed trait pattern
            let sealed_ident = Ident::new(&format!("{}Sealed", st.ident), st.ident.span());

            // Identifier for sealed trait containing aliases for bundle field types
            let fields_trait_ident = Ident::new(&format!("{}Fields", st.ident), st.ident.span());

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

            // Identifier for field enum (used as const generic)
            let field_enum_ident = Ident::new(&format!("{}Field", st.ident), st.ident.span());

            match st.fields {
                Fields::Named(FieldsNamed {
                    brace_token: _,
                    ref named,
                }) => {
                    let named = named.into_iter();
                    let fields_trait_inner = named.clone().map(|x| {
                        Ident::new(
                            &AsPascalCase(x.ident.clone().unwrap().to_string()).to_string(),
                            x.ident.clone().unwrap().span(),
                        )
                    });
                    let fields_enum_inner = fields_trait_inner.clone();

                    let fields_trait_impl = named.clone().map(|x| {
                        let field_ident = Ident::new(&AsPascalCase(x.ident.clone().unwrap().to_string()).to_string(), x.ident.clone().unwrap().span());
                        let field_ty = &x.ty;
                        quote! {
                        type #field_ident =
                            #mod_ident::#component_ident<{ #mod_ident::#field_enum_ident::#field_ident }, #field_ty>;
                        }
                    });

                    let bundle_inner_ident = named.clone().map(|x| {
                        let field_ident = x.ident.clone();
                        let field_upper_ident = Ident::new(&AsPascalCase(x.ident.clone().unwrap().to_string()).to_string(), x.ident.clone().unwrap().span());
                        let field_ty = &x.ty;
                        quote! {
                            #field_ident: #component_ident<{ #field_enum_ident::#field_upper_ident }, #field_ty>,
                        }
                    });

                    let from_inner = named.clone().map(|x| {
                        let field_ident = x.ident.clone();
                        let field_upper_ident = Ident::new(&AsPascalCase(x.ident.clone().unwrap().to_string()).to_string(), x.ident.clone().unwrap().span());
                        quote! {
                            #field_ident: #component_ident::<{ #field_enum_ident::#field_upper_ident }, _>(item.#field_ident),
                        }
                    });

                    quote! {
                    trait #sealed_ident {}
                    pub trait #fields_trait_ident: #sealed_ident {
                        #(type #fields_trait_inner;)*
                    }

                    #[automatically_derived]
                    impl #sealed_ident for #ident {}

                    #[automatically_derived]
                    impl #fields_trait_ident for #ident {
                        #(#fields_trait_impl)*
                    }


                    pub mod #mod_ident {
                        #[derive(Bundle)]
                        pub struct #bundle_ident {
                            #(#bundle_inner_ident)*
                        }

                        pub enum #field_enum_ident {
                            #(#fields_enum_inner,)*
                        }

                        #[automatically_derived]
                        impl From<#ident> for #bundle_ident {
                            fn from(item: #ident) -> #bundle_ident {
                                #bundle_ident {
                                    #(#from_inner)*
                                }
                            }

                        }

                        //#[derive(Deref, DerefMut)]
                        #[derive(Deref, DerefMut, Component)]
                        pub struct #component_ident<const FIELD: #field_enum_ident, T>(pub(super) T);
                    }
                    }.into()
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

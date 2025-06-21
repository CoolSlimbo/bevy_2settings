#![allow(clippy::let_and_return)]
#![deny(clippy::unwrap_used)]

mod stolen_code;

use darling::{
    FromDeriveInput, FromField, FromMeta,
    util::{Callable, Flag},
};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::ops::Deref;
use syn::{
    DeriveInput, Error, ExprStruct, Ident, Lit, Meta,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
};
use syn::{Expr, ExprLit};

// // Given a function, give the Ok variant, or return an error
// macro_rules! return_err {
//     ($e:expr) => {
//         match $e {
//             Ok(v) => v,
//             Err(e) => return e.to_compile_error().into(),
//         }
//     };
// }

// macro_rules! return_darling_err {
//     ($e:expr) => {
//         match $e {
//             Ok(v) => v,
//             Err(e) => return e.write_errors().into(),
//         }
//     };
// }

#[proc_macro_derive(Settings, attributes(settings))]
pub fn settings_derive_macro(input: TokenStream) -> TokenStream {
    derive_settings(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn derive_settings(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let input: DeriveInput = syn::parse(input)?;
    let input_ident = &input.ident;

    let derive_args: TopLevelSettings = FromDeriveInput::from_derive_input(&input)?;

    let syn::Data::Struct(s) = input.data else {
        return Err(syn::Error::new_spanned(input, "Only structs are supported"));
    };

    let mut field_meta = Vec::new();

    let mut partial_struct_fields = Vec::new();
    let mut partial_impls = Vec::new();

    let mut view_struct_fields = Vec::new();
    let mut view_impls = Vec::new();
    for field in s.fields.iter() {
        let FieldSettings {
            ty,
            // attrs,
            serde,
            ident,
            name,
            styling,
            default,
            predicate,
            nested,
        } = FromField::from_field(field)?;
        let field_docs = stolen_code::get_docs(&field.attrs, false)?;
        let ident = ident.ok_or(syn::Error::new_spanned(
            field,
            "Only named fields are supported",
        ))?;
        let nested = nested.into();

        let internal_name = ident.to_string();
        field_meta.push(quote! {
            ::bevy_2settings::Field {
                name: #name,
                internal_name: #internal_name,
                description: #field_docs,
                styling: &[#( ::bevy_2settings::Styling::#styling ),*],
                nested: #nested,
            }
        });

        let ty = type_to_ident(&ty)?;
        let (partial_ident, view_ident) = get_altnerate_idents(&ty);

        let partial_serde_attr = match serde {
            Some(SerdePassthrow(meta)) => quote! { #[ #meta ] },
            None => quote! {},
        };

        // Generate struct fields
        if nested {
            // If nested, use the ident of the nested struct
            partial_struct_fields.push(quote! {
                #partial_serde_attr
                #ident: #partial_ident,
            });
            view_struct_fields.push(quote! {
                #ident: #view_ident,
            });
        } else {
            partial_struct_fields.push(quote! {
                #partial_serde_attr
                #ident: ::std::option::Option<#ty>,
            });
            view_struct_fields.push(quote! {
                #ident: bool,
            });
        }
        // Implement trait

        // from_partial field implementors
        let f = if nested {
            quote! {
                #ident: <#ty as ::bevy_2settings::Settings>::from_partial(partial. #ident, &mut *world)?,
            }
        } else {
            match default {
                Some(MultiVal::Literal(l)) => quote! {
                    #ident: match partial. #ident {
                        Some(v) => v,
                        None => #l,
                    },
                },
                Some(MultiVal::Struct(s)) => quote! {
                    #ident: match partial. #ident {
                        Some(v) => v,
                        None => #s,
                    },
                },
                Some(MultiVal::Function(c)) => quote! {
                    #ident: match partial. #ident {
                        Some(v) => v,
                        None => ::bevy_2settings::RunSystemOnce::run_system_once(&mut *world, #c)??,
                    },
                },
                None => quote! {
                    #ident: match partial. #ident {
                        Some(v) => v,
                        None => <#ty as Default>::default(),
                    },
                },
            }
        };
        partial_impls.push(f);

        // get_view_tree field implementors
        let f = if nested {
            // quote! {
            //     #ident: <#ty as ::bevy_2settings::Settings>::get_view_tree(&self.#ident, &mut *world)?,
            // }
            quote! {
                #ident: self.nested.get_view_tree(&mut *world)?,
            }
        } else {
            match predicate {
                Some(MultiVal::Literal(ExprLit {
                    lit: Lit::Bool(syn::LitBool { value, .. }),
                    ..
                })) => {
                    quote!(
                        #ident: #value,
                    )
                }
                Some(MultiVal::Function(c)) => {
                    quote! {
                        #ident: ::bevy_2settings::RunSystemOnce::run_system_once(
                            &mut *world,
                            #c
                        )??,
                    }
                }
                None => {
                    quote! {
                        #ident: true,
                    }
                }
                Some(v) => return Err(syn::Error::new(v.span(), "Unsupported predicate")),
            }
        };
        view_impls.push(f);
    }

    let styles = derive_args.styling;
    let meta = quote! {
        const META: ::bevy_2settings::Meta = ::bevy_2settings::Meta {
            fields: &[#( #field_meta ),*],
            self_stylings: &[#( ::bevy_2settings::Styling::#styles ),*],
        };
    };

    let settings_name;
    if let Some(renamed) = derive_args.rename {
        settings_name = renamed;
    } else {
        settings_name = input_ident.to_string();
    }
    let settings_name = quote! {
        const INTERNAL_NAME: &'static str = #settings_name;
    };

    let (partial_ident, view_ident) = get_altnerate_idents(input_ident);
    let structual_impl = quote! {
        #[allow(warnings, clippy::all)]
        #[derive(Debug, Clone, Default, ::bevy_2settings::Deserialize)]
        #[serde(default)]
        struct #partial_ident {
            #( #partial_struct_fields )*
        }

        #[allow(warnings, clippy::all)]
        #[derive(Debug, Clone)]
        struct #view_ident {
            #( #view_struct_fields )*
        }
    };

    let from_partial_impl = quote! {
        fn from_partial(partial: Self::Partial, world: &mut ::bevy_2settings::World) -> ::std::result::Result<Self, ::bevy_2settings::BevyError> {
            Ok(Self {
                #( #partial_impls )*
            })
        }
    };

    let get_view_tree_impl = quote! {
        fn get_view_tree(&self, world: &mut ::bevy_2settings::World) -> ::std::result::Result<Self::ViewNode, ::bevy_2settings::BevyError> {
            Ok(Self::ViewNode {
                #( #view_impls )*
            })
        }
    };

    let out = quote! {
        #structual_impl
        impl ::bevy_2settings::Settings for #input_ident {
            #meta
            #settings_name
            type Partial = #partial_ident;
            type ViewNode = #view_ident;
            #from_partial_impl
            #get_view_tree_impl
        }
    };

    Ok(out)
}

fn get_altnerate_idents(ident: &Ident) -> (Ident, Ident) {
    let partial = format_ident!("__bevy_2settings_partial_{}", ident);
    let view = format_ident!("__bevy_2settings_view_{}", ident);
    (partial, view)
}

fn type_to_ident(ty: &syn::Type) -> syn::Result<Ident> {
    match ty {
        syn::Type::Path(p) => {
            let last = p
                .path
                .segments
                .last()
                .ok_or(syn::Error::new_spanned(ty, "Unsupported type"))?;
            Ok(last.ident.clone())
        }
        _ => Err(syn::Error::new_spanned(ty, "Unsupported type")),
    }
}

#[derive(Debug, Clone, Default)]
struct IdentList(Vec<Ident>);

impl Deref for IdentList {
    type Target = Vec<Ident>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parse for IdentList {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let punctuated = Punctuated::<Ident, Comma>::parse_terminated(input)?;
        Ok(IdentList(punctuated.into_iter().collect()))
    }
}

impl FromMeta for IdentList {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        syn::parse2::<IdentList>(item.require_list()?.tokens.clone())
            .map_err(darling::Error::custom)
    }
}

#[derive(Debug, Clone)]
struct SerdePassthrow(syn::Meta);

impl Parse for SerdePassthrow {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let meta = input.parse::<syn::Meta>()?;
        Ok(SerdePassthrow(meta))
    }
}

impl FromMeta for SerdePassthrow {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        Ok(SerdePassthrow(item.clone()))
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(settings))]
struct TopLevelSettings {
    #[darling(default)]
    styling: IdentList,
    rename: Option<String>,
}

#[derive(Debug, FromField)]
#[darling(attributes(settings))]
struct FieldSettings {
    ty: syn::Type,
    // attrs: Vec<syn::Attribute>,
    serde: Option<SerdePassthrow>,
    ident: Option<syn::Ident>,
    name: String,
    #[darling(default)]
    styling: IdentList,
    default: Option<MultiVal>,
    predicate: Option<MultiVal>,
    nested: Flag,
}

#[derive(Debug)]
enum MultiVal {
    Literal(ExprLit),
    Struct(ExprStruct),
    Function(Callable),
}

impl FromMeta for MultiVal {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        match Expr::from_meta(item) {
            Ok(Expr::Lit(l)) => Ok(MultiVal::Literal(l)),
            Ok(Expr::Struct(s)) => Ok(MultiVal::Struct(s)),
            Ok(_) => match Callable::from_meta(item) {
                Ok(c) => Ok(MultiVal::Function(c)),
                Err(e) => Err(e),
            },
            _ => Err(darling::Error::custom("Invalid kind of default value")),
        }
    }
}

impl MultiVal {
    fn span(&self) -> proc_macro2::Span {
        match self {
            MultiVal::Literal(l) => l.span(),
            MultiVal::Struct(s) => s.span(),
            MultiVal::Function(c) => c.span(),
        }
    }
}

use core::panic;

use proc_macro2::{self, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, DeriveInput, Expr, LitInt, Meta, Token, Variant,
};

#[proc_macro_derive(ErrorStatus, attributes(status))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);

    let ident = &ast.ident;
    let variants: &Punctuated<Variant, Token![,]> = match ast.data {
        syn::Data::Enum(ref s) => &s.variants,
        _ => panic!("ErrorResponse can only be applied on enums."),
    };

    let default_status_code = quote! {
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    };

    let cases: TokenStream = variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let expr = match variant
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("status"))
            {
                Some(attr) => match attr.clone().meta {
                    Meta::List(list) => {
                        if let Ok(number) = list.parse_args::<LitInt>() {
                            quote! {
                                axum::http::StatusCode::from_u16(#number as u16).unwrap()
                            }
                        } else if let Ok(expr) = list.parse_args::<Expr>() {
                            quote! {
                                #expr
                            }
                        } else {
                            default_status_code.clone()
                        }
                    }
                    _ => default_status_code.clone(),
                },
                None => default_status_code.clone(),
            };

            quote! {
                Self::#variant_ident => (#expr, format!("{}", self)).into_response(),
            }
        })
        .collect();

    let output = quote! {
        impl axum::response::IntoResponse for #ident {
            fn into_response(self) -> axum::response::Response {
                match self {
                    #cases
                }
            }
        }
    };
    output.into()
}

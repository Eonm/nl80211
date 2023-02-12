extern crate proc_macro;
use proc_macro::TokenStream;
use quote::ToTokens;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput};

use darling::{ast, FromDeriveInput, FromField};
use syn::spanned::Spanned;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(fmt), supports(struct_newtype))]
struct NlTypeArgs {
    #[darling(default)]
    before: Option<String>,
    #[darling(default)]
    after: Option<String>,
    #[darling(default)]
    cast: Option<darling::util::SpannedValue<String>>,
    fmt: Option<String>,
    data: ast::Data<(), darling::util::SpannedValue<ItemStructField>>,
}

#[derive(Debug, FromField)]
struct ItemStructField {
    ty: syn::Type,
}

#[proc_macro_derive(NlType, attributes(fmt))]
pub fn derive_nltype(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let args: NlTypeArgs = NlTypeArgs::from_derive_input(&derive_input).unwrap();
    let struct_ident = &derive_input.ident;

    let struct_ty = &args
        .data
        .as_ref()
        .take_struct()
        .expect("a struct type is expected")
        .fields[0]
        .ty;

    let try_from_impl = match struct_ty.clone().into_token_stream().to_string().as_ref() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "f32" | "i64" | "u64" | "f64" | "i128"
        | "u128" => {
            quote!(
                impl From<#struct_ty> for #struct_ident {
                    fn from(value: #struct_ty) -> Self {
                        #struct_ident(value)
                    }
                }

                impl std::convert::TryFrom<&[u8]> for #struct_ident {
                    type Error = std::array::TryFromSliceError;

                    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                        Ok(#struct_ident(#struct_ty::from_le_bytes(
                            std::convert::TryInto::try_into(value)?,
                        )))
                    }
                }
            )
        }
        "std::string::String" | "string::String" | "String" => {
            quote!(
                impl From<&[u8]> for #struct_ident {
                    fn from(value: &[u8]) -> Self {
                        #struct_ident(String::from_utf8_lossy(value).into_owned())
                    }
                }

                impl From<String> for #struct_ident {
                    fn from(value: String) -> Self {
                        #struct_ident(value)
                    }
                }

                impl From<&str> for #struct_ident {
                    fn from(value: &str) -> Self {
                        #struct_ident(value.to_string())
                    }
                }
            )
        }
        "[u8 ; 6]" => {
            quote! {
                impl From<[u8 ; 6]> for #struct_ident {
                    fn from(value: [u8 ; 6]) -> Self {
                        #struct_ident(value)
                    }
                }

                impl std::convert::TryFrom<&[u8]> for #struct_ident {
                    type Error = std::array::TryFromSliceError;

                    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                        Ok(#struct_ident(
                            std::convert::TryInto::try_into(value)?,
                        ))
                    }
                }
            }
        }
        _ => {
            quote_spanned! {struct_ty.span()=>
                compile_error!("This type is not supported by NlType");
            }
        }
    };

    let self_value = match args.cast {
        Some(ident) => {
            let cast_fn_ident = syn::Ident::new(ident.as_ref(), ident.span());

            quote!(
                #cast_fn_ident(self.0)
            )
        }
        None => {
            quote!(self.0)
        }
    };

    let formatter = args.fmt.unwrap_or_else(|| "{}".to_string());

    let display_write_statement = match (args.before, args.after) {
        (Some(before), Some(after)) => {
            let mut fmt_str = String::new();
            fmt_str.push_str("{}");
            fmt_str.push_str(&formatter);
            fmt_str.push_str("{}");

            let fmt_str_tokens = quote!(#fmt_str);

            quote!(write!(f, #fmt_str_tokens, #before, #self_value, #after))
        }
        (Some(before), None) => {
            let mut fmt_str = String::new();
            fmt_str.push_str("{}");
            fmt_str.push_str(&formatter);

            let fmt_str_tokens = quote!(#fmt_str);

            quote!(write!(f, #fmt_str_tokens, #before, #self_value))
        }
        (None, Some(after)) => {
            let mut fmt_str = String::new();
            fmt_str.push_str(&formatter);
            fmt_str.push_str("{}");

            let fmt_str_tokens = quote!(#fmt_str);

            quote!(write!(f, #fmt_str_tokens, #self_value, #after))
        }
        _ => {
            let mut fmt_str = String::new();
            fmt_str.push_str(&formatter);

            let fmt_str_tokens = quote!(#fmt_str);

            quote!(write!(f, #fmt_str_tokens, #self_value))
        }
    };

    quote!(
        #try_from_impl

        impl #struct_ident {
            pub fn inner(&self) -> &#struct_ty {
                &self.0
            }
        }

        impl std::fmt::Display for #struct_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #display_write_statement
            }
        }
    )
    .into()
}

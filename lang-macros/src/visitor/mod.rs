use crate::utils::fields_is_tuple;
use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::quote;
use syn::parse_macro_input;
use syn::AttributeArgs;
use syn::ItemEnum;

#[derive(FromMeta, Debug)]
struct OptionParser {
    #[darling(default)]
    with_mut: bool,
}

pub fn run(attr: TokenStream, item: TokenStream) -> TokenStream {
    let enum_item = parse_macro_input!(item as ItemEnum);
    let attr = parse_macro_input!(attr as AttributeArgs);

    let args = match OptionParser::from_list(&attr) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let mut output = vec![generate(&enum_item, false)];

    if args.with_mut {
        output.push(generate(&enum_item, true));
    }

    quote!(
        #enum_item

        #(
            #output
        )*
    )
    .into()
}

fn generate(enum_item: &ItemEnum, mutating: bool) -> TokenStream2 {
    let visitor_name = if mutating {
        format_ident!("{}VisitorMut", enum_item.ident)
    } else {
        format_ident!("{}Visitor", enum_item.ident)
    };

    let reference = if mutating { quote!(&mut ) } else { quote!(&) };

    let enum_name = format_ident!("{}", enum_item.ident.to_string().to_lowercase());

    let methods = enum_item.variants.iter().map(|variant| {
        let method_name = if mutating {
            format_ident!(
                "visit_mut_{}_{}",
                variant.ident.to_string().to_lowercase(),
                enum_name
            )
        } else {
            format_ident!(
                "visit_{}_{}",
                variant.ident.to_string().to_lowercase(),
                enum_name
            )
        };

        let is_tuple = fields_is_tuple(&variant.fields);

        let fields = variant.fields.iter().map(|field| {
            let ty = &field.ty;

            if let Some(name) = &field.ident {
                quote!(
                    #name: #reference #ty
                )
            } else {
                quote!(
                    #ty
                )
            }
        });

        let fields = if is_tuple {
            if variant.fields.len() == 1 {
                quote!(
                    member: #reference #(#fields),*
                )
            } else {
                quote!(
                    member: #reference (#(#fields),*)
                )
            }
        } else {
            quote!(#(#fields),*)
        };

        quote!(
            fn #method_name(&mut self, #fields) -> Self::Output;
        )
    });

    let name = &enum_item.ident;

    let accept = enum_item.variants.iter().map(|variant| {
        let name = &variant.ident;

        let tuple = fields_is_tuple(&variant.fields);

        let fields = variant
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                if let Some(name) = &field.ident {
                    quote!(
                        #name
                    )
                } else {
                    let name = format_ident!("field_{}", idx);

                    quote!(
                        #name
                    )
                }
            })
            .collect::<Vec<_>>();

        let method_name = if mutating {
            format_ident!(
                "visit_mut_{}_{}",
                variant.ident.to_string().to_lowercase(),
                enum_name
            )
        } else {
            format_ident!(
                "visit_{}_{}",
                variant.ident.to_string().to_lowercase(),
                enum_name
            )
        };

        if tuple {
            quote!(
                Self::#name(#(#fields),*) => visitor.#method_name(#(#fields),*)
            )
        } else {
            quote!(
                Self::#name { #(#fields),* } => visitor.#method_name(#(#fields),*)
            )
        }
    });

    let (generics_impl, generics_type, where_clause) = &enum_item.generics.split_for_impl();

    let accept_method = if mutating {
        format_ident!("accept_mut")
    } else {
        format_ident!("accept")
    };

    quote!(

        pub trait #visitor_name #generics_type #where_clause {
            type Output;

            #(#methods)*
        }


        impl #generics_impl  #name #generics_type #where_clause {
            pub fn #accept_method<V: #visitor_name #generics_type>(#reference self, visitor: &mut V) -> V::Output {
                match self {
                    #(#accept),*
                }
            }
        }


    )
}

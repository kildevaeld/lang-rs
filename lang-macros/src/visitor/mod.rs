use crate::utils::fields_is_tuple;
use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemEnum;

pub fn run(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let enum_item = parse_macro_input!(item as ItemEnum);

    let visitor_name = format_ident!("{}Visitor", enum_item.ident);

    let enum_name = format_ident!("{}", enum_item.ident.to_string().to_lowercase());

    let methods = enum_item.variants.iter().map(|variant| {
        let method_name = format_ident!(
            "visit_{}_{}",
            variant.ident.to_string().to_lowercase(),
            enum_name
        );

        let is_tuple = fields_is_tuple(&variant.fields);

        let fields = variant.fields.iter().map(|field| {
            let ty = &field.ty;

            if let Some(name) = &field.ident {
                quote!(
                    #name: &mut #ty
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
                    member: &mut #(#fields),*
                )
            } else {
                quote!(
                    member: &mut (#(#fields),*)
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

        let method_name = format_ident!(
            "visit_{}_{}",
            variant.ident.to_string().to_lowercase(),
            enum_name
        );

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

    quote!(
        #enum_item

        pub trait #visitor_name #generics_type #where_clause {
            type Output;

            #(#methods)*
        }

        impl #generics_impl #name #generics_type #where_clause {
            pub fn accept<V: #visitor_name #generics_type>(&mut self, visitor: &mut V) -> V::Output {
                match self {
                    #(#accept),*
                }
            }
        }


    )
    .into()
}

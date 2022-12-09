use proc_macro2::{Group, Ident, TokenStream};
use syn::{
    parse::{Parse, Parser},
    punctuated::Punctuated,
    Token, Type,
};

#[derive(Debug)]
pub struct AstType {
    pub name: Type,
    pub variants: Vec<AstTypeItem>,
}

impl Parse for AstType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let group = input.parse::<Group>()?;
        let parser = Punctuated::<AstTypeItem, Token![,]>::parse_terminated;

        let variants = parser.parse2(group.stream())?.into_iter().collect();

        Ok(AstType { name, variants })
    }
}

#[derive(Debug)]
pub struct AstTypeItem {
    pub name: Type,
    pub fields: Vec<AstTypeItemField>,
}

impl Parse for AstTypeItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let group = input.parse::<Group>()?;
        let parser = Punctuated::<AstTypeItemField, Token![,]>::parse_terminated;

        let fields = parser.parse2(group.stream())?.into_iter().collect();

        Ok(AstTypeItem { name, fields })
    }
}

#[derive(Debug)]
pub struct AstTypeItemField {
    pub name: Ident,
    pub value: Type,
}

impl Parse for AstTypeItemField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _ = input.parse::<Token![:]>()?;
        let value = input.parse()?;

        Ok(AstTypeItemField { name, value })
    }
}

pub fn parse(stream: TokenStream) -> Vec<AstType> {
    let parser = Punctuated::<AstType, Token![,]>::parse_terminated;
    let types = parser
        .parse2(stream)
        .expect("types")
        .into_iter()
        .collect::<Vec<_>>();

    types
}

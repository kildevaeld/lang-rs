use proc_macro2::{Group, Ident, Literal};
use syn::{parse::Parse, Token};

pub struct Assign {
    pub name: Ident,
    pub literal: Literal,
}

impl Parse for Assign {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let _punt = input.parse::<Token![=]>()?;
        let literal = input.parse::<Literal>()?;
        Ok(Assign { name, literal })
    }
}

pub struct Doc {
    pub literal: Literal,
}

impl Parse for Doc {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _token = input.parse::<Token![#]>()?;

        let group = input.parse::<Group>()?;

        let assign: Assign = syn::parse2(group.stream())?;

        Ok(Doc {
            literal: assign.literal,
        })
    }
}

use proc_macro2::{Group, Ident};
use syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    LitStr, Path, Token, Type,
};

#[derive(Debug, Clone)]
pub struct TypeList {
    pub types: Vec<Type>,
}

impl Parse for TypeList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let group = input.parse::<Group>()?;

        let parser = Punctuated::<Type, Token![,]>::parse_terminated;
        let types = parser.parse2(group.stream())?.into_iter().collect();

        Ok(TypeList { types })
    }
}

#[derive(Debug, Clone)]
pub struct Pair {
    pub token: LitStr,
    pub name: Ident,
}

impl Parse for Pair {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let token = input.parse()?;
        let name = input.parse()?;

        Ok(Pair { token, name })
    }
}

#[derive(Debug, Clone)]
pub struct PairGroup {
    pub pairs: Vec<Pair>,
}

impl Parse for PairGroup {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let group = input.parse::<Group>()?;
        let parser = Punctuated::<Pair, Token![,]>::parse_terminated;

        let pairs = parser.parse2(group.stream())?.into_iter().collect();

        Ok(PairGroup { pairs })
    }
}

#[derive(Debug, Clone)]
pub struct Tokens {
    // pub name: Ident,
    pub module_path: Option<Path>,
    pub puncts: PairGroup,
    pub keywords: PairGroup,
    pub literals: Option<TypeList>,
}

pub fn parse(input: ParseStream) -> syn::Result<Tokens> {
    // let name = input
    //     .parse::<Ident>()
    //     .map_err(|_| input.error("should start with name of Token"))?;
    let mut module_path = None;
    let mut puncts = None;
    let mut keywords = None;
    let mut literals = None;

    if input.peek(syn::Ident) && input.peek2(Token![:]) {
        if input.parse::<Ident>()?.to_string().as_str() != "module_path" {
            return Err(input.error("invalid indent"));
        }
        let _ = input.parse::<Token![:]>()?;

        let path = input.parse::<Path>()?;

        module_path = Some(path)
    }

    loop {
        if input.is_empty() {
            break;
        }

        let name = input.parse::<Ident>()?;
        let name_string = name.to_string();

        match name_string.as_str() {
            "punct" | "puncts" | "punctuation" | "Punct" => {
                puncts = Some(input.parse::<PairGroup>()?);
            }
            "keywords" | "keyword" | "Keyword" => {
                keywords = Some(input.parse::<PairGroup>()?);
            }
            "literal" => {
                literals = Some(input.parse::<TypeList>()?);
            }
            _ => {
                return Err(input.error(format!(
                    "expected oneof: keyword, punct or literal. found: {name_string}",
                )))
            }
        }

        if input.peek(Token![,]) {
            let _ = input.parse::<Token!(,)>()?;
        }
    }

    let puncts = puncts.unwrap();
    let keywords = keywords.unwrap();

    Ok(Tokens {
        module_path,
        puncts,
        keywords,
        literals,
    })
}

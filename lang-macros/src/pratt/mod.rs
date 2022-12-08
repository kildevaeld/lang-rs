use self::codegen::Input;
use self::parse::RuleList;
use proc_macro2::{Ident, TokenStream};
use syn::{
    parse::{ParseStream, Parser},
    Block, Token,
};

mod codegen;
mod parse;

fn parse(stream: ParseStream) -> syn::Result<(Ident, Ident, Option<Block>, Vec<RuleList>)> {
    let fn_name = stream.parse::<Ident>()?;
    let _ = stream.parse::<Token![->]>()?;
    let ret_name = stream.parse::<Ident>()?;

    let mut primary = None;

    let mut precedence_list = Vec::default();
    loop {
        if stream.is_empty() {
            break;
        }

        if let Ok(block) = stream.parse::<Block>() {
            primary = Some(block);
            break;
        }

        let rule_list = stream.parse::<RuleList>()?;

        precedence_list.push(rule_list);

        if stream.is_empty() {
            break;
        }

        let _ = stream.parse::<Token![-]>()?;
        let _ = stream.parse::<Token![-]>()?;
    }

    Ok((fn_name, ret_name, primary, precedence_list))
}

pub fn run(stream: TokenStream) -> TokenStream {
    let (mod_name, return_type, primary, rules) = parse.parse2(stream).expect("parse:");

    let output = codegen::run(Input {
        module: mod_name,
        return_type,
        primary,
        rules,
    });

    output
}

use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::{Group, Ident, Literal};
use quote::quote;
use syn::{
    parse::{Parse, Parser},
    punctuated::Punctuated,
    Token,
};

use super::shared::*;

#[derive(Debug, Clone)]
enum Arg {
    Take(u32),
    Returns(u32),
    Var(Variable),
}

#[derive(Debug, Clone)]
enum Variable {
    Constant,
    Local,
    Offset,
}

impl Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>().expect("name").to_string();

        let get_count = || {
            let _punt = input.parse::<Token![:]>()?;
            let count = input.parse::<Literal>()?;

            let count: u32 = count.to_string().parse().unwrap();

            syn::Result::Ok(count)
        };

        let ret = match ident.as_str() {
            "take" => Arg::Take(get_count()?),
            "out" => Arg::Returns(get_count()?),
            "offset" => Arg::Var(Variable::Offset),
            "constant" => Arg::Var(Variable::Constant),
            "local" => Arg::Var(Variable::Local),
            _ => panic!("invalid name"),
        };

        Ok(ret)
    }
}

#[derive(Debug, Clone)]

struct OpcodeItem {
    doc: Option<Literal>,
    name: Ident,
    take: u32,
    returns: u32,
    args: Option<Variable>,
}

impl Parse for OpcodeItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let doc = if input.peek(Token![#]) {
            Some(input.parse::<Doc>()?.literal)
        } else {
            None
        };

        let name = input.parse::<Ident>().expect("NAME");
        let call_group = input.parse::<Group>()?;

        let parser = Punctuated::<Arg, Token![,]>::parse_terminated;

        let args = parser.parse2(call_group.stream())?;

        let mut out = OpcodeItem {
            doc,
            name,
            take: 0,
            returns: 0,
            args: None,
        };

        for arg in args {
            match arg {
                Arg::Returns(ret) => {
                    out.returns = ret;
                }
                Arg::Take(take) => {
                    out.take = take;
                }
                Arg::Var(arg) => {
                    out.args = Some(arg);
                }
            }
        }

        Ok(out)
    }
}

pub fn run(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<OpcodeItem, Token![,]>::parse_terminated;

    let opcodes = match parser.parse(input) {
        Ok(ret) => ret,
        Err(err) => return err.into_compile_error().into(),
    };

    let opcodes_count = opcodes.len() as u8;

    let items = opcodes.iter().map(|item| {
        //
        let name = &item.name;
        let doc = item.doc.clone().unwrap_or_else(|| Literal::string(""));

        quote!(
            #[doc = #doc]
            #name
        )
    });

    let to_string = opcodes.iter().map(|item| {
        let name = &item.name;

        let str_rep = Literal::string(&format!(
            "OP_{}",
            name.to_string().to_screaming_snake_case()
        ));

        quote!(
            Opcode::#name => #str_rep
        )
    });

    let display = opcodes.iter().map(|item| {
        //
        let name = &item.name;
        let args: usize = match &item.args {
            Some(Variable::Constant) => 1,
            Some(Variable::Local) => 1,
            Some(Variable::Offset) => 2,
            None => 0,
        };

        let w = match &item.args {
            Some(Variable::Constant) => {
                quote!(
                    write!(writer, "{:0>5} {:<20} {:>5} {:?}", ip, self, opcodes[ip+1], constants[opcodes[ip+1] as usize])?;
                )
            }
            Some(Variable::Local) => quote!(
                write!(writer, "{:0>5} {:<20} {:>5}", ip, self, opcodes[ip +1])?;
            ),
            Some(Variable::Offset) => quote!(
                let offset = byteorder::BE::read_u16(&opcodes[ip+1..]);
                write!(writer, "{:0>5} {:<20} {} => {}", ip, self, ip, ip + offset as usize);
            ),
            None => quote!(
                write!(writer, "{:0>5} {:20}", ip, self)?;
            ),
        };

        quote!(
            Opcode::#name => {
                #w
                ip + 1 + #args
            }
        )
    });

    quote!(



        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, gc_arena::Collect)]
        #[collect(no_drop)]
        #[repr(u8)]
        pub enum Opcode {
            #(#items),*
        }

        impl Opcode {

            pub const COUNT: u8 = #opcodes_count;

            pub fn as_str(&self) -> &str {
                match self {
                    #(#to_string),*
                }
            }

            pub fn dissemble<'gc, W: ::core::fmt::Write>(&self, opcodes: &[u8], constants: &[crate::Value<'gc>], ip: usize, writer: &mut W) -> ::core::result::Result<usize, ::core::fmt::Error> {
                
                use byteorder::ByteOrder;

                let ip = match self {
                    #(#display),*
                };

                Ok(ip)
            }
        }


        impl ::core::fmt::Display for Opcode {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
               f.write_str(self.as_str())
            }
        }


        impl From<Opcode> for u8 {
            fn from(opcode: Opcode) -> u8 {
                opcode as u8
            }
        }

        #[derive(Debug)]
        pub struct TryFromErr;

        #[cfg(feature = "std")]
        impl ::std::error::Error for TryFromErr {

        }

        #[cfg(feature = "std")]
        impl ::core::fmt::Display for TryFromErr {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.write_str("invalid opcode")
             }
        }


        impl ::core::convert::TryFrom<u8> for Opcode {
            type Error = TryFromErr;
            fn try_from(i: u8) -> Result<Opcode, Self::Error> {
                if i >= #opcodes_count {
                    Err(TryFromErr)
                } else {
                    Ok(unsafe { ::core::mem::transmute(i)})
                }
            }
        }


    )
    .into()
}

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro2::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote;
use syn::{parse::Parse, Expr, Token};

#[proc_macro_hack]
pub fn btmap_proc(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ts2: TokenStream = TokenStream::from(ts);
    let iter = syn::parse2::<ExprList>(ts2).unwrap();

    let keys = iter.keys;
    let values = iter.values;

    let tokens = quote::quote! { {
        let mut tmp = std::collections::BTreeMap::new();
        #(
            tmp.insert(#keys, #values);
        )*
        tmp
    }
    };
    tokens.into()
}

struct ExprList {
    keys: Vec<Expr>,
    values: Vec<Expr>,
}

impl Parse for ExprList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut keys = Vec::new();
        let mut values = Vec::new();
        loop {
            let expr = input.parse::<Expr>().unwrap();
            keys.push(expr);

            if input.peek(Token![,]) {
                input.parse::<syn::token::Comma>().unwrap();
            }

            let expr = input.parse::<Expr>().unwrap();
            values.push(expr);
            if input.is_empty() {
                break;
            }

            if input.peek(Token![,]) {
                input.parse::<syn::token::Comma>().unwrap();
            }
        }
        let expr_list = ExprList {
            keys: keys,
            values: values,
        };
        syn::Result::Ok(expr_list)
    }
}

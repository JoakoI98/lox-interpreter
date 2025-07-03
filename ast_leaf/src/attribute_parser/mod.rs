use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::LinkedList;

use syn::{
    parenthesized,
    parse::{Lookahead1, Parse, ParseStream},
    punctuated::Punctuated,
    token, Ident, LitStr, Result, Token,
};

mod grouped;
mod production_token;
use production_token::ProductionToken;

#[derive(Debug)]
struct ProductionTokenChain {
    tokens: Vec<ProductionToken>,
}

impl Parse for ProductionTokenChain {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut tokens_list: LinkedList<ProductionToken> = LinkedList::new();
        while !input.is_empty() {
            if input.peek(token::Paren) {
                break;
            }
            let token = input.parse::<ProductionToken>()?;
            tokens_list.push_back(token);
        }
        let tokens: Vec<ProductionToken> = tokens_list.into_iter().collect();
        return Ok(Self { tokens });
    }
}

#[derive(Debug)]
enum ProductionItem {
    Group(grouped::Group),
    ProductionTokenChain(ProductionTokenChain),
}

impl Parse for ProductionItem {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(token::Paren) {
            let group = input.parse::<grouped::Group>()?;
            return Ok(ProductionItem::Group(group));
        }
        let production_token_chain = input.parse::<ProductionTokenChain>()?;
        return Ok(ProductionItem::ProductionTokenChain(production_token_chain));
    }
}

impl ProductionItem {
    pub fn get_enum_field(&self) -> TokenStream {
        match self {
            ProductionItem::Group(group) => group.get_enum_field(),
            ProductionItem::ProductionTokenChain(_) => quote! {},
        }
    }
}

#[derive(Debug)]
pub struct Production {
    items: Vec<ProductionItem>,
}

impl Parse for Production {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Err(syn::Error::new(input.span(), "Expected production"));
        }
        let mut items_list: LinkedList<ProductionItem> = LinkedList::new();
        while !input.is_empty() {
            let item = input.parse::<ProductionItem>()?;
            items_list.push_back(item);
        }
        let items: Vec<ProductionItem> = items_list.into_iter().collect();
        return Ok(Self { items });
    }
}

impl Production {
    pub fn get_enum(&self, name: &str) -> TokenStream {
        let items_tokens: Vec<TokenStream> = self
            .items
            .iter()
            .map(|item| item.get_enum_field())
            .collect();
        if items_tokens.is_empty() {
            return quote! {};
        }
        let enum_name_str = name.to_string() + "_Type";
        let enum_name = Ident::new(&enum_name_str, Span::call_site());
        return quote! {
            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
            pub enum #enum_name {
                #(#items_tokens)*
            }
        };
    }
}

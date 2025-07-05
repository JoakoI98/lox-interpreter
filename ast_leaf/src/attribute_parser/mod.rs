use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::LinkedList;

use syn::{
    parse::{Parse, ParseStream},
    token, Ident, Result, Type,
};

mod grouped;
mod production_token;
pub mod type_wrapper;
use production_token::ProductionToken;

use crate::struct_parser;

#[derive(Debug)]
struct ProductionTokenChain {
    tokens: Vec<ProductionToken>,
}

impl ProductionTokenChain {
    pub fn hydrate(self, name: &str, ty: Type) -> ProductionTokenChain {
        let tokens = self
            .tokens
            .into_iter()
            .map(|token| token.hydrate(name, ty.clone()))
            .collect();
        ProductionTokenChain { tokens }
    }

    pub fn get_parse_sentence(&self) -> TokenStream {
        let tokens = self.tokens.iter().map(|token| token.get_parse_sentence());
        quote! { #(#tokens)* }
    }

    pub fn get_peek1(&self) -> TokenStream {
        self.tokens[0].get_peek1()
    }
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

    pub fn hydrate(self, name: &str, ty: Type) -> ProductionItem {
        match self {
            ProductionItem::Group(group) => ProductionItem::Group(group.hydrate(name, ty.clone())),
            ProductionItem::ProductionTokenChain(production_token_chain) => {
                ProductionItem::ProductionTokenChain(
                    production_token_chain.hydrate(name, ty.clone()),
                )
            }
        }
    }

    pub fn get_parse_sentence(&self, enum_name: &str) -> TokenStream {
        match self {
            ProductionItem::Group(group) => group.get_parse_sentence(enum_name),
            ProductionItem::ProductionTokenChain(production_token_chain) => {
                production_token_chain.get_parse_sentence()
            }
        }
    }

    pub fn get_peek1(&self) -> TokenStream {
        match self {
            ProductionItem::Group(group) => group.get_peek1(),
            ProductionItem::ProductionTokenChain(production_token_chain) => {
                production_token_chain.get_peek1()
            }
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
    pub fn get_enum(&self, enum_name: &str) -> TokenStream {
        let items_tokens: Vec<TokenStream> = self
            .items
            .iter()
            .map(|item| item.get_enum_field())
            .collect();
        if items_tokens.is_empty() {
            return quote! {};
        }
        let enum_name_ident = Ident::new(enum_name, Span::call_site());
        return quote! {
            #[derive(Debug, PartialEq, Eq, Clone)]
            pub enum #enum_name_ident {
                #(#items_tokens)*,
                None,
            }
        };
    }

    pub fn hydrate(self, name: &str, ty: Type) -> Production {
        let items = self
            .items
            .into_iter()
            .map(|item| item.hydrate(name, ty.clone()))
            .collect();
        Production { items }
    }

    pub fn get_parse_sentence(&self, struct_ast: &struct_parser::ASTLeafStruct) -> TokenStream {
        let struct_name = struct_ast.name.to_string();
        let struct_name_ident = Ident::new(&struct_name, Span::call_site());

        let type_field_ident = &struct_ast.type_field_ident;

        let type_field_type_ident = Ident::new(&struct_ast.type_field, Span::call_site());

        let non_terminal_fields = struct_ast.non_terminal_fields.iter().map(|(name, _)| {
            let name_ident = Ident::new(name, Span::call_site());
            quote! {
                #name_ident
            }
        });

        let peek1 = self.items[0].get_peek1();

        let items = self
            .items
            .iter()
            .map(|item| item.get_parse_sentence(&struct_ast.type_field));
        quote! {

            impl Parser for #struct_name_ident {

                fn parse(input: &mut ParseStream) -> Result<Self> {
                    let mut type_variant = #type_field_type_ident::None ;
                    #(#items)*
                    std::result::Result::Ok(Self {
                        #type_field_ident: type_variant,
                        #(#non_terminal_fields),*
                    })
                }

                fn peek(input: &ParseStream) -> bool {
                    #peek1
                }

            }
        }
    }

    pub fn get_peek1(&self) -> TokenStream {
        self.items[0].get_peek1()
    }
}

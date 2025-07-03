use quote::quote;
use std::collections::LinkedList;

use proc_macro2::TokenStream;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Ident, LitInt, Result, Token,
};

use super::production_token::{ProductionToken, UnhydratedNonTerminal};

#[derive(Debug)]
struct InGroupProduction {
    tokens: Vec<ProductionToken>,
    store_index: u8,
}

impl Parse for InGroupProduction {
    fn parse(input: ParseStream) -> Result<Self> {
        let store_index_token = input.parse::<LitInt>()?;
        input.parse::<Token![:]>()?;
        let mut token_list: LinkedList<ProductionToken> = LinkedList::new();
        while !input.is_empty() {
            let token = input.parse::<ProductionToken>()?;
            token_list.push_back(token);
        }
        let tokens: Vec<ProductionToken> = token_list.into_iter().collect();
        let store_index: u8 = store_index_token.base10_parse()?;

        if (store_index as usize) >= tokens.len() {
            return Err(syn::Error::new(
                store_index_token.span(),
                "Store index is greater than the number of tokens",
            ));
        }

        return Ok(Self {
            tokens,
            store_index,
        });
    }
}

impl InGroupProduction {
    pub fn get_enum_field(&self) -> TokenStream {
        self.tokens[self.store_index as usize].get_enum_field()
    }
}

#[derive(Debug)]
enum GroupOrElement {
    ProductionToken(ProductionToken),
    InGroupProduction(InGroupProduction),
}

impl Parse for GroupOrElement {
    fn parse(input: ParseStream) -> Result<Self> {
        let multiple_next = input.peek(LitInt);
        if multiple_next {
            let production = input.parse::<InGroupProduction>()?;
            Ok(GroupOrElement::InGroupProduction(production))
        } else {
            let token = input.parse::<ProductionToken>()?;
            Ok(GroupOrElement::ProductionToken(token))
        }
    }
}

impl GroupOrElement {
    pub fn get_enum_field(&self) -> TokenStream {
        match self {
            GroupOrElement::ProductionToken(token) => token.get_enum_field(),
            GroupOrElement::InGroupProduction(production) => production.get_enum_field(),
        }
    }
}

#[derive(Debug)]
struct GroupedOr {
    elements: Vec<GroupOrElement>,
}

impl Parse for GroupedOr {
    fn parse(input: ParseStream) -> Result<Self> {
        let elements = Punctuated::<GroupOrElement, Token![|]>::parse_terminated(input)?;
        let elements_vec: Vec<GroupOrElement> = elements.into_iter().collect();
        return Ok(Self {
            elements: elements_vec,
        });
    }
}

impl GroupedOr {
    pub fn get_enum_field(&self) -> TokenStream {
        let types_tokens: Vec<TokenStream> = self
            .elements
            .iter()
            .map(|element| element.get_enum_field())
            .collect();

        return quote! {
                #(#types_tokens),*
        };
    }
}

#[derive(Debug)]
enum GroupPostfix {
    Star,
    Plus,
    Question,
    None,
}

#[derive(Debug)]
pub struct Group {
    or_elements: Option<GroupedOr>,
    non_terminal: Option<UnhydratedNonTerminal>,
    postfix: GroupPostfix,
}

impl Parse for Group {
    fn parse(input: ParseStream) -> Result<Self> {
        let main_content;
        parenthesized!(main_content in input);
        if main_content.peek2(Token![|]) {
            let or_elements = Some(main_content.parse::<GroupedOr>()?);
            return Ok(Self {
                or_elements,
                non_terminal: None,
                postfix: GroupPostfix::None,
            });
        }
        let mut or_elements = None;
        if main_content.peek(token::Paren) {
            let or_elements_content;
            parenthesized!(or_elements_content in main_content);
            or_elements = Some(or_elements_content.parse::<GroupedOr>()?);
        }
        let mut non_terminal = None;
        if main_content.peek(Ident) {
            non_terminal = Some(main_content.parse::<UnhydratedNonTerminal>()?);
        }
        if non_terminal.is_none() && or_elements.is_none() {
            return Err(syn::Error::new(
                main_content.span(),
                "Expected non-terminal or or-grouped elements",
            ));
        }
        let mut postfix = GroupPostfix::None;
        if input.peek(Token![*]) {
            postfix = GroupPostfix::Star;
            input.parse::<Token![*]>()?;
        } else if input.peek(Token![+]) {
            postfix = GroupPostfix::Plus;
            input.parse::<Token![+]>()?;
        } else if input.peek(Token![?]) {
            postfix = GroupPostfix::Question;
            input.parse::<Token![?]>()?;
        }
        return Ok(Self {
            or_elements,
            non_terminal,
            postfix,
        });
    }
}

impl Group {
    pub fn get_enum_field(&self) -> TokenStream {
        if let Some(or_elements) = &self.or_elements {
            return or_elements.get_enum_field();
        }
        return quote! {};
    }
}

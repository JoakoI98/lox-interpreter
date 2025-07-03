use quote::quote;
use std::collections::LinkedList;

use proc_macro2::TokenStream;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Ident, LitInt, Result, Token, Type,
};

use crate::attribute_parser::production_token::HydratedNonTerminal;

use super::production_token::{NonTerminal, ProductionToken};

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

    pub fn hydrate(self, name: &str, ty: Type) -> InGroupProduction {
        let tokens = self
            .tokens
            .into_iter()
            .map(|token| match &token {
                ProductionToken::NonTerminal(NonTerminal::Unhydrated(unhydrated)) => {
                    let type_ident = Ident::new(&unhydrated.name, unhydrated.span);
                    let ty = Type::Verbatim(quote! { #type_ident });
                    ProductionToken::NonTerminal(NonTerminal::Hydrated(HydratedNonTerminal {
                        name: unhydrated.name.clone(),
                        span: unhydrated.span,
                        ty,
                    }))
                }
                _ => token,
            })
            .collect();
        InGroupProduction {
            tokens,
            store_index: self.store_index,
        }
    }

    pub fn get_parse_sentence(&self) -> TokenStream {
        let tokens = self.tokens.iter().map(|token| token.get_parse_sentence());
        quote! { #(#tokens)* }
    }

    pub fn get_peek1(&self) -> TokenStream {
        self.tokens[0].get_peek1()
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

    pub fn hydrate(self, name: &str, ty: Type) -> GroupOrElement {
        match self {
            GroupOrElement::ProductionToken(token) => {
                GroupOrElement::ProductionToken(token.hydrate(name, ty.clone()))
            }
            GroupOrElement::InGroupProduction(production) => {
                GroupOrElement::InGroupProduction(production.hydrate(name, ty.clone()))
            }
        }
    }

    pub fn get_parse_sentence(&self) -> TokenStream {
        match self {
            GroupOrElement::ProductionToken(token) => token.get_parse_sentence(),
            GroupOrElement::InGroupProduction(production) => production.get_parse_sentence(),
        }
    }

    pub fn get_peek1(&self) -> TokenStream {
        match self {
            GroupOrElement::ProductionToken(token) => token.get_peek1(),
            GroupOrElement::InGroupProduction(production) => production.get_peek1(),
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

    pub fn hydrate(self, name: &str, ty: Type) -> GroupedOr {
        let elements = self
            .elements
            .into_iter()
            .map(|element| element.hydrate(name, ty.clone()))
            .collect();
        GroupedOr { elements }
    }

    pub fn get_parse_sentence(&self) -> TokenStream {
        let peek1 = self.elements.iter().map(|element| element.get_peek1());
        let match_tokens: Vec<TokenStream> = self
            .elements
            .iter()
            .map(|element| element.get_parse_sentence())
            .collect();
        let enum_variants: Vec<TokenStream> = self
            .elements
            .iter()
            .map(|element| element.get_enum_field())
            .collect();

        let mut i: usize = 0;
        let if_tokens = peek1.map(|peek| {
            let match_token = &match_tokens[i];
            let enum_variant = &enum_variants[i];
            i += 1;
            quote! {
                if #peek {
                    type_variant = PrimaryExpressionType::#enum_variant;
                    #match_token
                }
            }
        });

        return quote! {
            #(#if_tokens)else*
        };
    }

    pub fn get_peek1(&self) -> TokenStream {
        let peek1_tokens = self.elements.iter().map(|element| element.get_peek1());
        quote! { #(#peek1_tokens)||* }
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
    non_terminal: Option<NonTerminal>,
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
            non_terminal = Some(main_content.parse::<NonTerminal>()?);
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

    pub fn hydrate(self, name: &str, ty: Type) -> Group {
        let or_elements = self
            .or_elements
            .map(|or_elements| or_elements.hydrate(name, ty.clone()));
        let non_terminal = self
            .non_terminal
            .map(|non_terminal| non_terminal.hydrate(name, ty.clone()));
        Group {
            or_elements,
            non_terminal,
            postfix: self.postfix,
        }
    }

    pub fn get_parse_sentence(&self) -> TokenStream {
        let or_elements = self
            .or_elements
            .as_ref()
            .map(|or_elements| or_elements.get_parse_sentence())
            .unwrap_or(quote! {});
        let non_terminal = self
            .non_terminal
            .as_ref()
            .map(|non_terminal| non_terminal.get_parse_sentence())
            .unwrap_or(quote! {});
        return quote! {
            #or_elements
            #non_terminal
        };
    }

    pub fn get_peek1(&self) -> TokenStream {
        match &self.or_elements {
            Some(or_elements) => or_elements.get_peek1(),
            None => self
                .non_terminal
                .as_ref()
                .map(|non_terminal| non_terminal.get_peek1())
                .unwrap_or(quote! {}),
        }
    }
}

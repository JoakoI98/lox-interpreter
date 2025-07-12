use quote::quote;
use std::collections::LinkedList;

use proc_macro2::{Span, TokenStream};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Ident, LitInt, Result, Token, Type,
};

use crate::attribute_parser::{production_token::HydratedNonTerminal, type_wrapper::TypeWrapper};

use super::production_token::{NonTerminal, ProductionToken};

pub fn to_snake_case(name: &str) -> String {
    let mut snake_case = String::new();
    for (i, c) in name.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            snake_case.push('_');
        }
        snake_case.push(c.to_ascii_lowercase());
    }
    snake_case
}

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

    pub fn get_enum_sentence(&self) -> TokenStream {
        self.tokens[self.store_index as usize].get_enum_sentence()
    }

    pub fn hydrate(self, _name: &str, _ty: Type) -> InGroupProduction {
        let tokens = self
            .tokens
            .into_iter()
            .map(|token| match &token {
                ProductionToken::NonTerminal(NonTerminal::Unhydrated(unhydrated)) => {
                    let type_ident = Ident::new(&unhydrated.name, unhydrated.span);
                    let ty = Type::Verbatim(quote! { #type_ident });
                    ProductionToken::NonTerminal(NonTerminal::Hydrated(HydratedNonTerminal {
                        name: to_snake_case(&unhydrated.name),
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

    pub fn get_enum_sentence(&self) -> TokenStream {
        match self {
            GroupOrElement::ProductionToken(token) => token.get_enum_sentence(),
            GroupOrElement::InGroupProduction(production) => production.get_enum_sentence(),
        }
    }

    pub fn hydrate(self, name: &str, ty: Type) -> GroupOrElement {
        match self {
            GroupOrElement::ProductionToken(ProductionToken::NonTerminal(
                NonTerminal::Unhydrated(unhydrated),
            )) => {
                let type_ident = Ident::new(&unhydrated.name, unhydrated.span);
                let ty = Type::Verbatim(quote! { #type_ident });
                GroupOrElement::ProductionToken(ProductionToken::NonTerminal(
                    NonTerminal::Hydrated(HydratedNonTerminal {
                        name: to_snake_case(&unhydrated.name),
                        span: unhydrated.span,
                        ty,
                    }),
                ))
            }
            GroupOrElement::ProductionToken(production_token) => {
                GroupOrElement::ProductionToken(production_token.hydrate(name, ty.clone(), false))
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

    pub fn get_parse_sentence(
        &self,
        enum_name: &str,
        type_variant_ident: Option<&Ident>,
        force_match: bool,
    ) -> TokenStream {
        let peek1 = self.elements.iter().map(|element| element.get_peek1());
        let match_tokens: Vec<TokenStream> = self
            .elements
            .iter()
            .map(|element| element.get_parse_sentence())
            .collect();
        let enum_name_ident = Ident::new(enum_name, Span::call_site());
        let enum_variants: Vec<TokenStream> = self
            .elements
            .iter()
            .map(|element| element.get_enum_sentence())
            .collect();

        let default_type_variant_ident = Ident::new("type_variant", Span::call_site());

        let type_variant_ident = type_variant_ident.unwrap_or(&default_type_variant_ident);

        let mut if_tokens = std::collections::LinkedList::<TokenStream>::new();

        for (i, peek) in peek1.enumerate() {
            let match_token = &match_tokens[i];
            let enum_variant = &enum_variants[i];
            let if_statement = if i < self.elements.len() - 1 || !force_match {
                quote! {
                    if #peek
                }
            } else {
                quote! {}
            };
            if_tokens.push_back(quote! {
                #if_statement {
                    #match_token
                    #type_variant_ident = #enum_name_ident::#enum_variant;
                }
            });
        }

        let last_if_tokens = if_tokens.pop_back();
        let if_tokens = if_tokens.into_iter();

        return quote! {
            #(#if_tokens)else*
            else #last_if_tokens
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

    fn get_parse_star(&self, enum_name: &str) -> TokenStream {
        let non_terminal = self.non_terminal.as_ref();
        if non_terminal.is_none() {
            return quote! {};
        }
        let or_elements = self.or_elements.as_ref();
        if or_elements.is_none() {
            return quote! {};
        }
        let non_terminal = non_terminal.unwrap();
        let non_terminal_name = Ident::new(non_terminal.get_name(), Span::call_site());
        let non_terminal_type = non_terminal.get_type();
        let peek_sentence = or_elements
            .map(|or_elements| or_elements.get_peek1())
            .unwrap_or(non_terminal.get_peek1());
        let type_variant_ident = Ident::new("current_type_variant", Span::call_site());
        let or_elements_parse_sentence = or_elements
            .map(|or_elements| {
                or_elements.get_parse_sentence(enum_name, Some(&type_variant_ident), false)
            })
            .unwrap_or(quote! {});
        let enum_ident = Ident::new(enum_name, Span::call_site());
        quote! {
            let mut #non_terminal_name = std::collections::LinkedList::new();
            while #peek_sentence {
                let mut #type_variant_ident = #enum_ident::None;
                #or_elements_parse_sentence
                let nt = input.parse::<#non_terminal_type>()?;
                #non_terminal_name.push_back((#type_variant_ident, nt));
            }
            let #non_terminal_name: Vec<_> = #non_terminal_name.into_iter().collect();
        }
    }

    fn get_parse_plus(&self, enum_name: &str) -> TokenStream {
        let non_terminal = self.non_terminal.as_ref();
        if non_terminal.is_none() {
            return quote! {};
        }
        let or_elements = self.or_elements.as_ref();
        if or_elements.is_none() {
            return quote! {};
        }
        let non_terminal = non_terminal.unwrap();
        let non_terminal_name = Ident::new(non_terminal.get_name(), Span::call_site());
        let non_terminal_type = non_terminal.get_type();
        let peek_sentence = or_elements
            .map(|or_elements| or_elements.get_peek1())
            .unwrap_or(non_terminal.get_peek1());
        let type_variant_ident = Ident::new("current_type_variant", Span::call_site());
        let or_elements_parse_sentence = or_elements
            .map(|or_elements| {
                or_elements.get_parse_sentence(enum_name, Some(&type_variant_ident), true)
            })
            .unwrap_or(quote! {});
        let enum_ident = Ident::new(enum_name, Span::call_site());
        quote! {
            let mut #non_terminal_name = std::collections::LinkedList::new();
            {
                let mut #type_variant_ident = #enum_ident::None;
                #or_elements_parse_sentence
                let nt = input.parse::<#non_terminal_type>()?;
                #non_terminal_name.push_back((#type_variant_ident, nt));
            }
            while #peek_sentence {
                let mut #type_variant_ident = #enum_ident::None;
                #or_elements_parse_sentence
                let nt = input.parse::<#non_terminal_type>()?;
                #non_terminal_name.push_back((#type_variant_ident, nt));
            }
            let #non_terminal_name: Vec<_> = #non_terminal_name.into_iter().collect();
        }
    }

    fn get_parse_question(&self, enum_name: &str) -> TokenStream {
        let non_terminal = self.non_terminal.as_ref();
        if non_terminal.is_none() {
            return quote! {};
        }
        let or_elements = self.or_elements.as_ref();
        if or_elements.is_none() {
            return quote! {};
        }
        let non_terminal = non_terminal.unwrap();
        let non_terminal_name = Ident::new(non_terminal.get_name(), Span::call_site());
        let non_terminal_type = non_terminal.get_type();
        let peek_sentence = or_elements
            .map(|or_elements| or_elements.get_peek1())
            .unwrap_or(non_terminal.get_peek1());
        let type_variant_ident = Ident::new("current_type_variant", Span::call_site());
        let or_elements_parse_sentence = or_elements
            .map(|or_elements| {
                or_elements.get_parse_sentence(enum_name, Some(&type_variant_ident), false)
            })
            .unwrap_or(quote! {});
        let enum_ident = Ident::new(enum_name, Span::call_site());
        quote! {
            let mut #non_terminal_name = None;
            let mut #type_variant_ident = #enum_ident::None;
            if #peek_sentence {
                #or_elements_parse_sentence
                #non_terminal_name = Some(input.parse::<#non_terminal_type>()?);
            }
            type_variant = #type_variant_ident;


        }
    }

    fn get_parse_non_terminal(&self) -> TokenStream {
        self.non_terminal
            .as_ref()
            .map(|non_terminal| non_terminal.get_parse_sentence())
            .unwrap_or(quote! {})
    }

    pub fn hydrate(self, name: &str, ty: Type, enum_name: &str) -> Group {
        let is_non_terminal = self
            .non_terminal
            .as_ref()
            .map(|non_terminal| non_terminal.get_name() == name)
            .unwrap_or(false);

        let mut type_to_hydrate = ty;
        if is_non_terminal {
            let type_wrapper = TypeWrapper::new(&type_to_hydrate);
            match self.postfix {
                GroupPostfix::Star | GroupPostfix::Plus => {
                    let inner_tuple = type_wrapper.validate_type("Vec", 0);
                    type_to_hydrate =
                        TypeWrapper::new(&inner_tuple).validate_dual_tuple(enum_name, None);
                }
                GroupPostfix::Question => {
                    type_to_hydrate = type_wrapper.validate_type("Option", 0);
                }
                _ => {}
            }
        }

        let or_elements = self
            .or_elements
            .map(|or_elements| or_elements.hydrate(name, type_to_hydrate.clone()));
        let non_terminal = self
            .non_terminal
            .map(|non_terminal| non_terminal.hydrate(name, type_to_hydrate.clone(), false));
        Group {
            or_elements,
            non_terminal,
            postfix: self.postfix,
        }
    }

    pub fn get_parse_no_postfix(&self, enum_name: &str) -> TokenStream {
        let or_elements = self
            .or_elements
            .as_ref()
            .map(|or_elements| or_elements.get_parse_sentence(enum_name, None, true))
            .unwrap_or(quote! {});
        let non_terminal = self.get_parse_non_terminal();
        quote! {
            #or_elements
            #non_terminal
        }
    }

    pub fn get_parse_sentence(&self, enum_name: &str) -> TokenStream {
        match &self.postfix {
            GroupPostfix::Star => self.get_parse_star(enum_name),
            GroupPostfix::Plus => self.get_parse_plus(enum_name),
            GroupPostfix::Question => self.get_parse_question(enum_name),
            GroupPostfix::None => self.get_parse_no_postfix(enum_name),
        }
    }

    pub fn get_peek1(&self) -> TokenStream {
        match &self.or_elements {
            Some(or_elements) => or_elements.get_peek1(),
            None => match &self.postfix {
                GroupPostfix::None | GroupPostfix::Plus => self
                    .non_terminal
                    .as_ref()
                    .map(|non_terminal| non_terminal.get_peek1())
                    .unwrap_or(quote! {}),
                _ => quote! {true},
            },
        }
    }
}

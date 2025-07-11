use core::str;
use std::fmt;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr, Result, Type,
};

#[derive(Debug)]
pub struct Terminal {
    value: String,
}

impl Parse for Terminal {
    fn parse(input: ParseStream) -> Result<Self> {
        let token = input.parse::<LitStr>()?;
        let token_value = token.value();
        let token_value = match token_value.as_str() {
            "(" => "LeftParen",
            ")" => "RightParen",
            "{" => "LeftBrace",
            "}" => "RightBrace",
            "," => "Comma",
            "." => "Dot",
            "-" => "Minus",
            "+" => "Plus",
            ";" => "Semicolon",
            "/" => "Slash",
            "*" => "Star",
            "!" => "Bang",
            "=" => "Equal",
            ">" => "Greater",
            "<" => "Less",
            "and" => "And",
            "class" => "Class",
            "else" => "Else",
            "false" => "False",
            "fun" => "Fun",
            "for" => "For",
            "if" => "If",
            "nil" => "Nil",
            "or" => "Or",
            "return" => "Return",
            "super" => "Super",
            "print" => "Print",
            "this" => "This",
            "true" => "True",
            "var" => "Var",
            "while" => "While",
            "==" => "EqualEqual",
            "!=" => "BangEqual",
            ">=" => "GreaterEqual",
            "<=" => "LessEqual",
            "STRING" => "String",
            "NUMBER" => "Number",
            "IDENT" => "Identifier",
            "ID" => "Identifier",
            "IDENTIFIER" => "Identifier",
            _ => return Err(syn::Error::new(token.span(), "Invalid token")),
        };
        Ok(Terminal {
            value: token_value.into(),
        })
    }
}

impl ToTokens for Terminal {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = Ident::new(&self.value, Span::call_site());
        tokens.extend(quote! { #ident });
    }
}

impl Terminal {
    pub fn get_parse_sentence(&self) -> TokenStream {
        let ident = Ident::new(&self.value, Span::call_site());
        quote! { tokens_list.push_back(input.parse::<#ident>()?.token.clone());}
    }

    pub fn get_peek1(&self) -> TokenStream {
        let ident = Ident::new(&self.value, Span::call_site());
        quote! { input.peek::<#ident>() }
    }
}

#[derive(Debug)]
pub struct UnhydratedNonTerminal {
    pub name: String,
    pub span: Span,
}

impl Parse for UnhydratedNonTerminal {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        let name = input.parse::<Ident>()?;

        return Ok(Self {
            name: name.to_string(),
            span,
        });
    }
}

impl ToTokens for UnhydratedNonTerminal {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = Ident::new(&self.name, self.span);
        tokens.extend(quote! { #ident });
    }
}

pub struct HydratedNonTerminal {
    pub name: String,
    pub span: Span,
    pub ty: Type,
}

impl fmt::Debug for HydratedNonTerminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HydratedNonTerminal {{ name: {}, span: {:?}, ty: {:?} }}",
            self.name,
            self.span,
            self.ty.to_token_stream()
        )
    }
}

#[derive(Debug)]
pub enum NonTerminal {
    Hydrated(HydratedNonTerminal),
    Unhydrated(UnhydratedNonTerminal),
}

impl Parse for NonTerminal {
    fn parse(input: ParseStream) -> Result<Self> {
        let unhydrated = input.parse::<UnhydratedNonTerminal>()?;
        Ok(NonTerminal::Unhydrated(unhydrated))
    }
}

impl NonTerminal {
    pub fn get_enum_field(&self) -> TokenStream {
        let hydrated = match self {
            NonTerminal::Hydrated(hydrated) => hydrated,
            NonTerminal::Unhydrated(_) => panic!("Unhydrated non-terminal found"),
        };
        let ty = &hydrated.ty;
        quote! { #ty(#ty) }
    }

    pub fn get_name(&self) -> &str {
        match self {
            NonTerminal::Hydrated(hydrated) => &hydrated.name,
            NonTerminal::Unhydrated(unhydrated) => &unhydrated.name,
        }
    }

    pub fn get_type(&self) -> &Type {
        match self {
            NonTerminal::Hydrated(hydrated) => &hydrated.ty,
            NonTerminal::Unhydrated(_) => panic!("Unhydrated non-terminal found"),
        }
    }

    pub fn get_enum_sentence(&self) -> TokenStream {
        let hydrated = match self {
            NonTerminal::Hydrated(hydrated) => hydrated,
            NonTerminal::Unhydrated(_) => panic!("Unhydrated non-terminal found"),
        };
        let name = Ident::new(&hydrated.name, Span::call_site());
        let ty = &hydrated.ty;
        quote! { #ty(#name) }
    }

    pub fn hydrate(self, name: &str, ty: Type, force: bool) -> NonTerminal {
        match &self {
            NonTerminal::Hydrated(_) => self,
            NonTerminal::Unhydrated(unhydrated) => {
                if unhydrated.name != name && !force {
                    return self;
                }
                let hydrated = HydratedNonTerminal {
                    name: unhydrated.name.clone(),
                    span: unhydrated.span,
                    ty,
                };
                NonTerminal::Hydrated(hydrated)
            }
        }
    }

    pub fn get_parse_sentence(&self) -> TokenStream {
        match self {
            NonTerminal::Hydrated(hydrated) => {
                let ident = Ident::new(&hydrated.name, hydrated.span);
                let ty = &hydrated.ty;
                quote! { let #ident = input.parse::<#ty>()?; }
            }
            NonTerminal::Unhydrated(_) => panic!("Unhydrated non-terminal found"),
        }
    }

    pub fn get_peek1(&self) -> TokenStream {
        match self {
            NonTerminal::Hydrated(hydrated) => {
                let ty = &hydrated.ty;
                quote! { input.peek::<#ty>() }
            }
            NonTerminal::Unhydrated(_) => panic!("Unhydrated non-terminal found"),
        }
    }
}

#[derive(Debug)]
pub enum ProductionToken {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}

impl Parse for ProductionToken {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            // Try to parse as UnhydratedNonTerminal
            let non_terminal = input.parse::<NonTerminal>()?;
            Ok(ProductionToken::NonTerminal(non_terminal))
        } else if lookahead.peek(LitStr) {
            // Try to parse as Terminal (assuming terminals are bracketed)
            let terminal = input.parse::<Terminal>()?;
            Ok(ProductionToken::Terminal(terminal))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ProductionToken {
    pub fn get_enum_field(&self) -> TokenStream {
        match self {
            ProductionToken::Terminal(terminal) => quote! { #terminal},
            ProductionToken::NonTerminal(non_terminal) => non_terminal.get_enum_field(),
        }
    }

    pub fn get_enum_sentence(&self) -> TokenStream {
        match self {
            ProductionToken::Terminal(terminal) => quote! { #terminal},
            ProductionToken::NonTerminal(non_terminal) => non_terminal.get_enum_sentence(),
        }
    }

    pub fn hydrate(self, name: &str, ty: Type, force: bool) -> ProductionToken {
        match self {
            ProductionToken::Terminal(terminal) => ProductionToken::Terminal(terminal),
            ProductionToken::NonTerminal(non_terminal) => {
                ProductionToken::NonTerminal(non_terminal.hydrate(name, ty, force))
            }
        }
    }

    pub fn get_parse_sentence(&self) -> TokenStream {
        match self {
            ProductionToken::Terminal(terminal) => terminal.get_parse_sentence(),
            ProductionToken::NonTerminal(non_terminal) => non_terminal.get_parse_sentence(),
        }
    }

    pub fn get_peek1(&self) -> TokenStream {
        match self {
            ProductionToken::Terminal(terminal) => terminal.get_peek1(),
            ProductionToken::NonTerminal(non_terminal) => non_terminal.get_peek1(),
        }
    }
}

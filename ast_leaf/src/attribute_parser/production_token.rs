use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr, Result,
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

#[derive(Debug)]
pub struct UnhydratedNonTerminal {
    name: String,
    span: Span,
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

#[derive(Debug)]
pub enum ProductionToken {
    Terminal(Terminal),
    NonTerminal(UnhydratedNonTerminal),
}

impl Parse for ProductionToken {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            // Try to parse as UnhydratedNonTerminal
            let non_terminal = input.parse::<UnhydratedNonTerminal>()?;
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
            ProductionToken::NonTerminal(non_terminal) => quote! { #non_terminal (#non_terminal)},
        }
    }
}

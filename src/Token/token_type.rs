use std::fmt::Display;
pub enum ArrangedTokens {
    Single(Box<dyn TokenType>),
    Multiple(Box<dyn TokenType>, Box<dyn TokenType>),
    Same,
}

pub trait TokenType: Display {
    fn literal_value(&self) -> Option<String> {
        None
    }

    fn arrange_token(&self, _lexeme: &str) -> Result<ArrangedTokens, super::scanner::TokenErrors> {
        Ok(ArrangedTokens::Same)
    }
}

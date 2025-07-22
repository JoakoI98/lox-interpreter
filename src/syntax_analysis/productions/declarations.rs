use std::fmt::Debug;

use ast_leaf::ast_leaf;

use super::super::parsing::primitives::{
    Class, Equal, Fun, Identifier, LeftBrace, Less, RightBrace, Semicolon, Var,
};
use super::super::parsing::{ParseStream, Parser, Result};
use super::assignments::Expression;

use super::statement::Statement;
use crate::syntax_analysis::productions::functions::Function;
use crate::tokenizer::Token;

#[ast_leaf("var" "IDENT" (("=") expr)? ";")]
#[derive(Debug, PartialEq, Clone)]
pub struct VarDeclaration {
    #[Type]
    pub token_type: VarDeclarationType,
    pub expr: Option<Expression>,
    #[TokenList]
    pub token_list: Vec<Token>,
}

#[ast_leaf("fun" function)]
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    #[Type]
    pub token_type: FunctionDeclarationType,
    pub function: Function,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SuperClassIdentifier {
    pub super_class: Option<Identifier>,
}

impl Parser for SuperClassIdentifier {
    fn parse(stream: &mut ParseStream) -> Result<Self> {
        let mut super_class = None;
        if stream.peek::<Less>() {
            stream.parse::<Less>()?;
            super_class = Some(stream.parse()?);
        }
        Ok(Self { super_class })
    }

    fn peek(_stream: &ParseStream) -> bool {
        true
    }
}

#[ast_leaf("class" "IDENT" super_class "{" (functions)* "}")]
#[derive(Debug, PartialEq, Clone)]
pub struct ClassDeclaration {
    #[Type]
    pub token_type: ClassDeclarationType,
    pub functions: Vec<(ClassDeclarationType, Function)>,
    pub super_class: SuperClassIdentifier,
    #[TokenList]
    pub token_list: Vec<Token>,
}

#[ast_leaf((VarDeclaration | Statement | FunctionDeclaration | ClassDeclaration))]
#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    #[Type]
    pub token_type: DeclarationType,
}

#[ast_leaf((statements)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct ProgramAst {
    #[Type]
    pub token_type: ProgramType,
    pub statements: Vec<(ProgramType, Declaration)>,
}

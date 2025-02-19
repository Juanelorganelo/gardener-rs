use crate::validation::Validation;

pub trait Parser<'a> {
    type Ast;
    type ParserError;

    fn parse(self, source: &'a str) -> Validation<Self::Ast, Self::ParserError>;
}

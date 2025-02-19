use regex::Regex;
use crate::parser::Parser;
use crate::validation::Validation;

pub struct Config<'a> {
    pattern: &'a str
}

pub struct Trunk<'a> {
    config: Config<'a>
}

pub enum Error {
    RegexError(regex::Error)
}

impl From<regex::Error> for Error {
    fn from(value: regex::Error) -> Self {
        Self::RegexError(value)
    }
}

pub enum Branch<'a> {
    JiraId(&'a str),
    Pattern(&'a str)
}

impl<'a> Parser<'a> for Trunk<'a> {
    type Ast = Branch<'a>;
    type ParserError = Error;

    fn parse(self, source: &'a str) -> Validation<Self::Ast, Self::ParserError> {
        todo!();
    }
}

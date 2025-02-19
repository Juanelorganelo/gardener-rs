use crate::validators::ticket_id::Tracker;
use crate::parser::Parser;
use crate::validation::Validation;

struct TicketId {
    tracker: Tracker,
    api_url: String,
    api_token: String
}

pub struct Config {
    pattern: String,
    ticket_id: Option<TicketId>
}

pub struct Trunk {
    config: Config
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

impl<'a> Parser<'a> for Trunk {
    type Ast = Branch<'a>;
    type ParserError = Error;

    fn parse(self, source: &'a str) -> Validation<Self::Ast, Self::ParserError> {
        todo!();
    }
}

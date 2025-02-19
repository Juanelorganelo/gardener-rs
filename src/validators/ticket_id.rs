use super::{Branch, Validator};
use crate::validation::Validation;

pub enum Tracker {
    Jira {
        api_url: String,
        api_token: String
    }
}

pub struct TicketIdValidator;

pub enum TicketIdValidationError {
    NotFound(String)
}

impl<'a> Validator<'a> for TicketIdValidator {
    type ValidationError = TicketIdValidationError;

    fn validate(&self, branch: Branch<'a>) -> Validation<Branch<'a>, Self::ValidationError> {
        todo!();
    }
}

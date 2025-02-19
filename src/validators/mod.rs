use crate::validation::Validation;

pub mod ticket_id;

pub struct Branch<'a> {
    branch_type: &'a str,
    description: &'a str,
    ticket_ids: Vec<Option<&'a str>>
}

pub trait Validator<'a> {
    type ValidationError;

    fn validate(&self, branch: Branch<'a>) -> Validation<Branch<'a>, Self::ValidationError>;
}

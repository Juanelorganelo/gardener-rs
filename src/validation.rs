/**
 * This is basically Result with error accumulation semantics.
 */
#[derive(Debug, PartialEq)]
pub enum Validation<T, E> {
    Valid(T),
    Invalid(Vec<E>)
}

impl<T, E> Validation<T, E> {
    pub fn valid(value: T) -> Self {
        Validation::Valid(value)
    }

    pub fn invalid(errors: Vec<E>) -> Self {
        Validation::Invalid(errors)
    }
}

impl<T, E> From<Result<T, E>> for Validation<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(value) => Self::Valid(value),
            Err(error) => Self::Invalid(vec![error])
        }
    }
}

impl<T, E> From<Result<T, Vec<E>>> for Validation<T, E> {
    fn from(value: Result<T, Vec<E>>) -> Self {
        match value {
            Ok(value) => Self::Valid(value),
            Err(error) => Self::Invalid(error)
        }
    }
}

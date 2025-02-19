pub mod trunk;

pub trait Workflow: Sized {
    fn config_key() -> &'static str;
}

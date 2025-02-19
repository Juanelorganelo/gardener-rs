use async_trait::async_trait;

#[async_trait]
pub trait Vcs {
    type Error;

    async fn detect() -> Option<Box<Self>>;
    async fn get_current_branch(&self) -> Result<String, Self::Error>;
}

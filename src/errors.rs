#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid ident: {0}")]
    Ident(String),
}

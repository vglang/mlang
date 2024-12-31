#[derive(thiserror::Error, Debug)]
pub enum Error<'a> {
    #[error("Invalid ident: {0}")]
    Ident(&'a str),
}

#[derive(thiserror::Error, Debug)]
pub enum Error<'a> {
    #[error("({1},{2}) Invalid ident: {0}")]
    Ident(&'a str, usize, usize),

    #[error("({0},{1}) Expect comment prefix: ///")]
    Comment(usize, usize),
}

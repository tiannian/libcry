#[derive(Debug)]
pub enum Error {}

pub type Result<T> = core::result::Result<T, Error>;

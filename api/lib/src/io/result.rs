pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    _message: String,
}

impl Error {
    pub fn internal_error() -> Self {
        Self {
            _message: "Something went wrong.".into(),
        }
    }
}

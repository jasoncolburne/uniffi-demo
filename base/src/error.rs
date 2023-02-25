pub type BoxedError = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, BoxedError>;    

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    External(String),
    #[error("{0}")]
    Common(String)
}

macro_rules! err {
    ($e:expr) => {
        Err(Box::new($e))
    };
}

impl From<BoxedError> for Error {
    fn from(boxed: BoxedError) -> Error {
        Error::External(boxed.to_string())
    }
}

pub(crate) use err;

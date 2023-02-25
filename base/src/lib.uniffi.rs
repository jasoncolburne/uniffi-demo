mod error;
mod module;

pub use error::Error;
use module::{Class, Uniffi, WrappingClass};

uniffi::include_scaffolding!("base");


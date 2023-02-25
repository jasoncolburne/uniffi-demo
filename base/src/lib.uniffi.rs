mod error;
mod module;

pub use error::Error;
pub use module::{Class, WrappingClass};
pub use module::Uniffi as ModuleInterface;

uniffi::include_scaffolding!("base");


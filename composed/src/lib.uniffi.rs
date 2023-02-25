mod error;
mod module;

pub use error::Error;
pub use module::Composed;

uniffi::include_scaffolding!("composed");

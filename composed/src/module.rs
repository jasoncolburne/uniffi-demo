use base::{WrappingClass, ModuleInterface};
use crate::error::Result;

pub struct Composed {
    original: WrappingClass,
    clone: WrappingClass
}

impl Composed {
    pub fn new(kind: &str, size: u32, data: &[u8]) -> Result<Composed> {
        let original = WrappingClass::new(None, Some(kind.to_string()), Some(size), Some(data.to_vec()))?;
        let clone = WrappingClass::new(Some(original.inner()), None, None, None)?;

        Ok(Composed { original, clone })
    }

    pub fn original(&self) -> std::sync::Arc<WrappingClass> {
        std::sync::Arc::new(self.original.clone())
    }

    pub fn clone(&self) -> std::sync::Arc<WrappingClass> {
        std::sync::Arc::new(self.clone.clone())
    }

    pub fn compare(&self, wrapping: Option<std::sync::Arc<WrappingClass>>) -> bool {
        if let Some(wrapping) = wrapping {
            return (*wrapping).inner() == (*self.original()).inner()
        } else {
            false
        }
    }
}
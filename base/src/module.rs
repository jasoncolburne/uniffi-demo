use crate::error::{err, Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    kind: String,
    size: u32,
    data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WrappingClass {
    class: Class
}

pub trait Creation {
    fn new_with_class(class: &Class) -> Result<Self> where Self: Sized;
    fn new_with_data(kind: &str, size: u32, data: &[u8]) -> Result<Self> where Self: Sized;
}

pub trait Exposed {
    fn wrapped(&self) -> Class;
}

pub trait ClassLike: Creation + Exposed {
    fn new(
        wrapped: Option<&Class>,
        kind: Option<&str>,
        size: Option<u32>,
        data: Option<&[u8]>,
    ) -> Result<Self> where Self: Sized {
        if let Some(class) = wrapped {
            Self::new_with_class(class)
        } else if kind.is_some() && size.is_some() && data.is_some() {
            Self::new_with_data(kind.unwrap(), size.unwrap(), data.unwrap())
        } else {
            return err!(Error::Common("must specify class or data".to_string()))
        }
    }

    fn inner(&self) -> Class {
        self.wrapped()
    }
}

pub trait Uniffi: Creation + Exposed {
    fn new(
        wrapped: Option<std::sync::Arc<Class>>,
        kind: Option<String>,
        size: Option<u32>,
        data: Option<Vec<u8>>,
    ) -> Result<Self> where Self: Sized {
        if let Some(class) = wrapped {
            let class = (*class).clone();
            Self::new_with_class(&class)
        } else if kind.is_some() && size.is_some() && data.is_some() {
            Self::new_with_data(&kind.unwrap(), size.unwrap(), &data.unwrap())
        } else {
            return err!(Error::Common("must specify class or data".to_string()))
        }
    }

    fn inner(&self) -> std::sync::Arc<Class> {
        let wrapped = self.wrapped();
        std::sync::Arc::new(wrapped)
    }
}

impl Default for Class {
    fn default() -> Self {
        Class { kind: "".to_string(), size: 0, data: vec![] }
    }
}

impl Class {
    pub fn kind(&self) -> String {
        self.kind.clone()
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl WrappingClass {
    pub fn explode(&self) -> Result<()> {
        err!(Error::Common("explosion!".to_string()))
    }
}

impl Exposed for WrappingClass {
    fn wrapped(&self) -> Class {
        self.class.clone()
    }
}

impl Creation for WrappingClass {
    fn new_with_class(class: &Class) -> Result<Self> {
        Ok(WrappingClass { class: class.clone() })
    }

    fn new_with_data(kind: &str, size: u32, data: &[u8]) -> Result<Self> {
        let class = Class { kind: kind.to_string(), size, data: data.to_vec() };
        Ok(WrappingClass { class })
    }
}

impl ClassLike for WrappingClass {}
impl Uniffi for WrappingClass {}

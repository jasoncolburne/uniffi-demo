pub mod error {
    pub type BoxedError = Box<dyn std::error::Error>;
    pub type Result<T> = core::result::Result<T, BoxedError>;    

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
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
            boxed.into()
        }
    }

    pub(crate) use err;
}

mod module {
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
        fn create(class: Option<&Class>) -> Result<Self> where Self: Sized;
    }

    pub trait Exposed {
        fn class(&self) -> Class;
    }

    pub trait ClassLike: Creation + Exposed {
        fn new(class: Option<&Class>) -> Result<Self> where Self: Sized {
            Self::create(class)
        }
    }

    pub(crate) trait Uniffi: ClassLike {
        fn new(class: std::sync::Arc<Class>) -> Result<Self> where Self: Sized {
            <Self as ClassLike>::new(Some(&class))
        }

        fn class(&self) -> std::sync::Arc<Class> {
            std::sync::Arc::new(<Self as Exposed>::class(self))
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
            err!(Error::Common("generic error".to_string()))
        }
    }

    impl Exposed for WrappingClass {
        fn class(&self) -> Class {
            self.class.clone()
        }
    }

    impl Creation for WrappingClass {
        fn create(class: Option<&Class>) -> Result<Self> {
            let class = class.map_or(Class::default(), |class| class.clone());
            Ok(WrappingClass { class })
        }
    }

    impl ClassLike for WrappingClass {}
    impl Uniffi for WrappingClass {}
}

pub use error::Error;
use module::{Class, Uniffi, WrappingClass};

uniffi::include_scaffolding!("base");


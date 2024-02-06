use std::fmt::Display;

//shared/common GL types
pub mod types {
    pub type GLbyte = std::ffi::c_char;
    pub type GLchar = std::ffi::c_char;
    pub type GLbitField = std::ffi::c_uint;
    pub type GLenum = std::ffi::c_uint;
    pub type GLfixed = GLint;
    pub type GLfloat = std::ffi::c_float;
    pub type GLhalf = std::ffi::c_ushort;
    pub type GLint = std::ffi::c_int;
    pub type GLintptr = isize;
    pub type GLsizei = std::ffi::c_int;
    pub type GLsizeiptr = isize;
    pub type GLuint = std::ffi::c_uint;

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct GLboolean(std::ffi::c_uchar);

    impl GLboolean {
        pub const TRUE: Self = Self(1);
        pub const FALSE: Self = Self(0);
    }

    impl From<bool> for GLboolean {
        fn from(value: bool) -> Self {
            if value {
                Self::TRUE
            } else {
                Self::FALSE
            }
        }
    }
}

pub mod gl43;

#[derive(Debug)]
pub enum Error {
    FailedToLoad(String),
}

trait Loader {
    fn load<T>(&self, symbol: &str) -> Result<T, Error>;
}

impl<F> Loader for F
where
    F: Fn(&str) -> *const std::ffi::c_void,
{
    fn load<T>(&self, symbol: &str) -> Result<T, Error> {
        let ptr = (self)(symbol);

        if ptr.is_null() {
            Err(Error::FailedToLoad(symbol.to_string()))
        } else {
            Ok(unsafe { std::mem::transmute_copy(&ptr) })
        }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToLoad(s) => write!(f, "function pointer for {s} not found"),
        }
    }
}

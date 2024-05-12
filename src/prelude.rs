pub use crate::error::MyError;
pub use crate::error::ThisError;
pub type Result<T> = std::result::Result<T, MyError>;

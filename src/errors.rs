#![allow(dead_code, unused_imports)]
pub use crate::traits_traits::StringLike;
use std::fmt::Display;

#[derive(Debug)]
struct ErrType {
    message: String,
}

impl ErrType {
    fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "msg: {}", self.message)
    }
}

type MyResult = Result<String, u8>;

fn throw_s(val: u8) -> MyResult {
    Err(val)
}

fn ok_s(val: impl ToString) -> MyResult {
    Ok(val.to_string())
}

type ErrResult<T> = Result<T, ErrType>;

fn throw<T: ToString>(val: T) -> ErrResult<T> {
    Err(ErrType::new(val))
}

fn ok<T>(val: T) -> ErrResult<T> {
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert!(throw_s(2).is_err());
        assert!(ok_s("hi").is_ok());

        assert_eq!(ok_s("ok").unwrap(), "ok");

        let s = throw_s(5).unwrap_or_else(|e| e.to_string());
        assert_eq!(s, "5");
    }

    #[test]
    fn options() {
        let o: Option<String> = None;
        let result = o.ok_or(ErrType::new("not found"));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "not found");

        let o2: Option<&str> = Some("hi");
        let result2: Result<&str, ErrType> = o2.ok_or(ErrType::new("not found"));

        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), "hi");
    }

    #[test]
    fn extra_fun() {
        assert_eq!(ErrType::new("hi").stringify(), "msg: hi");

        let err = ErrType::new("hi");
        assert_eq!(StringLike::stringify(&err), "msg: hi");
    }
}

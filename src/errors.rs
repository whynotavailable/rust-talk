#![allow(dead_code, unused_imports)]
pub use crate::traits::StringLike2;
use std::fmt::Display;

#[derive(Debug)]
struct ErrType {
    message: String,
}

impl ErrType {
    fn new(message: impl ToString) -> Self {
        ErrType {
            message: message.to_string(),
        }
    }
}

impl Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "msg: {}", self.message)
    }
}

fn throw_s<T>(val: T) -> Result<T, T> {
    Err(val)
}

fn ok_s<T>(val: T) -> Result<T, T> {
    Ok(val)
}

fn throw<T: ToString>(val: T) -> Result<T, ErrType> {
    Err(ErrType::new(val))
}

fn ok<T>(val: T) -> Result<T, ErrType> {
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert!(throw_s("hi").is_err());
        assert!(ok_s("hi").is_ok());

        assert_eq!(ok_s(1).unwrap(), 1);

        let s: &str = throw_s("hi err").unwrap_or_else(|e| e);
        assert_eq!(s, "hi err");
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
        assert_eq!(ErrType::new("hi").stringify2(), "msg: hi")
    }
}

#![allow(dead_code, unused_imports)]
pub use crate::traits::StringLike;
use std::fmt::Display;

#[derive(Debug)]
pub struct ErrType {
    pub message: String,
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

type ErrResult<T> = Result<T, ErrType>;
pub type EmptyResult = ErrResult<()>;

fn throw<T: ToString>(val: T) -> ErrResult<T> {
    Err(ErrType::new(val))
}

fn ok<T>(val: T) -> ErrResult<T> {
    Ok(val)
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fmt::Debug};

    use super::*;

    #[test]
    fn basic() {
        assert!(throw(2).is_err());
        assert!(ok("hi").is_ok());

        assert_eq!(ok("ok").unwrap(), "ok");

        let s = throw(5).unwrap_or(42);
        assert_eq!(s, 42);
    }

    /// This method returns the page subtracted by one.
    fn get_page_string(param: Option<String>) -> i32 {
        let param: Option<i32> = param.map(|s| s.parse().unwrap_or(1));

        match param {
            Some(i) if i > 1 => i - 1,
            _ => 0,
        }
    }

    fn get_page_str(param: Option<&str>) -> i32 {
        let param: Option<i32> = param.map(|s| s.parse().unwrap_or(1));

        match param {
            Some(i) if i > 1 => i - 1,
            _ => 0,
        }
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

    fn some_num(s: &str) -> Option<String> {
        Some(s.to_string())
    }

    #[test]
    fn conditionals() {
        let op: Option<String> = Some("hi".to_string());
        if let Some(s) = op {
            println!("{}", s);
        }
    }

    fn wrap_bool(b: bool) -> Option<bool> {
        Some(b)
    }

    fn none_bool() -> Option<bool> {
        None
    }

    #[test]
    fn conditionals2() {
        assert!(none_bool().unwrap_or(true));
        assert!(wrap_bool(true).unwrap_or(true));
        assert!(!wrap_bool(false).unwrap_or(true));
    }

    #[test]
    fn paging() {
        assert_eq!(get_page_string(some_num("5")), 4);
        assert_eq!(get_page_string(some_num("0")), 0);
        assert_eq!(get_page_string(some_num("-5")), 0);
        assert_eq!(get_page_string(some_num("lol what")), 0);

        assert_eq!(get_page_string(None), 0);

        let s: Option<String> = some_num("1");
        assert_eq!(get_page_string(s), 0);
        // s is gone

        let s: Option<&str> = Some("1");
        assert_eq!(get_page_str(s), 0);
        assert_eq!(get_page_str(s), 0);
        assert!(s.is_some())
    }

    #[test]
    fn extra_fun() {
        assert_eq!(ErrType::new("hi").stringify(), "msg: hi");

        let err = ErrType::new("hi");
        assert_eq!(StringLike::stringify(&err), "msg: hi");
    }

    fn check_traits<T: Debug + Display>(_: T) {}

    #[test]
    fn traits() {
        check_traits(ErrType::new("hi"));
    }
}

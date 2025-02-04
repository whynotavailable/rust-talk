#![allow(dead_code)]

use std::fmt::Display;

// Traits

trait StringLike1 {
    fn stringify(&self) -> String;
}

pub trait StringLike2 {
    fn stringify2(&self) -> String;
}

// impl StringLike1

impl StringLike1 for String {
    fn stringify(&self) -> String {
        self.clone()
    }
}

impl StringLike1 for &str {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

// impl StringLike2

impl<T: ToString> StringLike2 for T {
    fn stringify2(&self) -> String {
        self.to_string()
    }
}

struct TestThing {
    data: String,
}

impl Display for TestThing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl TestThing {
    fn new(s: impl StringLike1) -> Self {
        TestThing {
            data: s.stringify(),
        }
    }

    fn new2<T: StringLike2>(s: T) -> TestThing {
        TestThing {
            data: s.stringify2(),
        }
    }

    #[allow(clippy::needless_lifetimes)]
    fn get_data<'a>(&'a self) -> &'a String {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        let cstr: &'static str = "hi";
        assert_eq!(TestThing::new(cstr).data, "hi");
        assert_eq!(TestThing::new2(cstr).data, "hi");

        assert_eq!(TestThing::new("hi".stringify()).data, "hi");
        assert_eq!(TestThing::new2("hi".stringify2()).data, "hi");
    }

    #[test]
    fn get_data() {
        let thing = TestThing::new("hi");

        assert_eq!(thing.get_data(), "hi");
        assert_eq!(TestThing::get_data(&thing), "hi");
    }
}

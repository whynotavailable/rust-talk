#![allow(dead_code)]

use std::fmt::Display;

trait StringLike {
    fn stringify(&self) -> String;
}

impl StringLike for String {
    fn stringify(&self) -> String {
        self.clone()
    }
}

impl StringLike for &str {
    fn stringify(&self) -> String {
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
    fn new(s: impl StringLike) -> Self {
        Self {
            data: s.stringify(),
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

        assert_eq!(TestThing::new("hi".stringify()).data, "hi");
    }

    #[test]
    fn get_data() {
        let thing = TestThing::new("hi");

        assert_eq!(thing.get_data(), "hi");
        assert_eq!(TestThing::get_data(&thing), "hi");
    }
}

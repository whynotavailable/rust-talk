#![allow(dead_code)]

pub trait StringLike {
    fn stringify(&self) -> String;
}

impl<T: ToString> StringLike for T {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

struct TestThing {
    data: String,
}

impl TestThing {
    fn new(s: impl StringLike) -> Self {
        Self {
            data: s.stringify(),
        }
    }

    fn get_data(&self) -> &String {
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

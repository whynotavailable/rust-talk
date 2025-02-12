fn main() {}

#[cfg(test)]
mod test {
    #[allow(dead_code)]
    struct Data<'a> {
        msg: &'a str,
    }

    #[allow(dead_code)]
    struct StringData {
        msg: String,
    }

    fn call<T>(s: T) -> usize {
        let raw_ptr: *const T = &s;

        raw_ptr as usize
    }

    #[test]
    fn no_copy() {
        let data: String = "hi".to_string();
        let pt: *const String = &data;
        let loc = pt as usize;

        let loc2 = call(data);
        assert_eq!(loc, loc2);
    }

    #[test]
    fn no_copy_struct() {
        let data = StringData {
            msg: "hi".to_string(),
        };

        let pt: *const StringData = &data;
        let loc = pt as usize;

        let loc2 = call(data);
        assert_eq!(loc, loc2);
    }

    #[test]
    fn copy_str() {
        let data: &str = "hi";
        let pt: *const &str = &data;
        let loc = pt as usize;

        let loc2 = call(data);
        assert_ne!(loc, loc2);
    }

    #[test]
    fn copy_int() {
        let data: i32 = 42;
        let pt: *const i32 = &data;
        let loc = pt as usize;

        let loc2 = call(data);
        assert_ne!(loc, loc2);
    }

    #[test]
    fn copy_struct() {
        let data = Data { msg: "hi" };

        let pt: *const Data = &data;
        let loc = pt as usize;

        let loc2 = call(data);
        assert_ne!(loc, loc2);
    }
}

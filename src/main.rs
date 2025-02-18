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

    fn get_page(page: Option<i16>) -> i16 {
        match page {
            Some(i) if i >= 1 => i - 1,
            _ => 0,
        }
    }

    #[test]
    fn test_page() {
        // TODO: test scenarios for Option.or, Option.ok, Some(thing) = what
        //
        assert_eq!(get_page(Some(5)), 4);
        assert_eq!(get_page(Some(1)), 0);
        assert_eq!(get_page(Some(0)), 0);
        assert_eq!(get_page(Some(-123)), 0);
        assert_eq!(get_page(None), 0);
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

    #[test]
    fn num() {
        let mut n: u32 = 12;
        let mut_ref = &mut n;
        *mut_ref = 32;

        assert_eq!(n, 32);
    }
}

fn main() {}

#[cfg(test)]
mod test {
    fn call_no_copy(s: String) -> usize {
        let raw_ptr: *const String = &s;

        raw_ptr as usize
    }

    #[test]
    fn no_copy() {
        let basic_str: String = "hi".to_string();
        let pt: *const String = &basic_str;
        let loc = pt as usize;

        let loc2 = call_no_copy(basic_str);
        assert_eq!(loc, loc2);
    }

    fn call_copy<T: Copy>(s: T) -> usize {
        let raw_ptr: *const T = &s;

        raw_ptr as usize
    }

    #[test]
    fn copy() {
        let basic_i: i32 = 42;
        let pt: *const i32 = &basic_i;
        let loc = pt as usize;

        let loc2 = call_copy(basic_i);
        assert_ne!(loc, loc2);
    }
}

#![allow(dead_code)]
fn root() -> String {
    let mut string_buff = String::new();

    string_buff.push('0');

    level1(&mut string_buff);
    level1(&mut string_buff);

    string_buff
}

fn level1(string_buff: &mut String) {
    string_buff.push('1');
    level2(string_buff);
    level2(string_buff);
}

fn level2(string_buff: &mut String) {
    string_buff.push('2');
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doubler(s: String) -> String {
        format!("{}{}", s, s)
    }

    #[test]
    fn basics() {
        let s = "hi".to_string();
        doubler(s);
        // - s no longer exists

        let s = "hi".to_string();
        let s = doubler(s);
        assert_eq!(s, "hihi");
    }

    #[test]
    fn pointers() {
        let results = root();
        assert_eq!(results, "0122122")
    }
}

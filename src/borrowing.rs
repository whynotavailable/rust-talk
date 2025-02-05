#![allow(dead_code)]
fn root() -> String {
    let mut string_buff = String::new();

    string_buff.push_str("root\n");

    level1(&mut string_buff);
    level1(&mut string_buff);

    string_buff
}

fn level1(string_buff: &mut String) {
    string_buff.push_str("-1\n");
    level2(string_buff);
    level2(string_buff);
}

fn level2(string_buff: &mut String) {
    string_buff.push_str("--2\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let results = root();
        assert_eq!(
            results,
            "root
-1
--2
--2
-1
--2
--2
"
        )
    }
}

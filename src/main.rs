extern crate regex;
use regex::Regex;

fn tokenize(s: &str) -> Vec<&str> {
    let re = Regex::new(r"[ \t]+").unwrap();
    let tokens: Vec<&str> = re.split(s).collect();
    return tokens;
}

#[test]
fn test_tokenize() {
    println!("in test_tokenize");
    let teststr1 = "key1 value1  key2  value2 ";
    let result1 = tokenize(teststr1);
    assert_eq!(result1, vec!("key1", "value1", "key2", "value2"));
}

fn main() {
    println!("Hello, world!");
}

fn tokenize(s: &str) {
    println!("in tokenize");
}

#[test]
fn test_tokenize() {
    println!("in test_tokenize");
    let teststr1 = "key1 value1  key2  value2 ";
    let result1 = tokenize(teststr1);
    assert!(true);
}

fn main() {
    println!("Hello, world!");
}

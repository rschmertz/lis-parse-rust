extern crate regex;
use regex::Regex;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::io::prelude::*;
use std::path::Path;

/*
    tokenizes a string in an EPOCH DB.  Assumptions:

    - string is of the form <key value key value ...>
    - key is a simple string of non-whitespace characters
    - value may be as above or a quote-delimited string
      poss. containing whitespace
*/
fn tokenize(s: &str) -> Vec<&str> {
    let re = Regex::new(" *\" *").unwrap();
    let quote_nonquotes = re.split(s);
    let mut tokens: Vec<&str> = vec!();
    let mut is_quoted_string = false; // first token, a key, is never quoted
    let space_re = Regex::new("[ \t]+").unwrap();
    for qnq in quote_nonquotes {
        if is_quoted_string  {
            tokens.push(qnq);
        } else {
            for subtoken in space_re.split(qnq) {
                tokens.push(subtoken);
            }
        }
        is_quoted_string = !is_quoted_string;
    }
    return tokens;
}

fn line_get(li: &mut Lines<BufReader<File>>) -> Option<String> {
    let r = li.next();
    // may need to clone string to return unmodified -- or not.....
    let t = r.unwrap().unwrap();
    Some(t)
}

#[test]
fn test_tokenize() {
    println!("in test_tokenize");
    let teststr1 = "key1 \"value 1\"  key2  value2 key3 \"value 3\"";
    let result1 = tokenize(teststr1);
    assert_eq!(result1, vec!("key1", "value 1", "key2", "value2", "key3", "value 3"));
    let teststr2 = "one two    three \t four";
    let result2 = tokenize(teststr2);
    assert_eq!(result2, vec!("one", "two", "three", "four"));
}

#[test]
fn test_line_get() {

    let path = Path::new("./test/testfile.lis");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let mut line_iter = reader.lines();

    let line = line_get(&mut line_iter);
    let s: String = line.unwrap();
    assert!(s.ends_with("six"));
}

fn main() {
    println!("Hello, world!");
}

use std::collections::HashMap;

pub fn run(part: i32, input: &str) {
    if part == 1 {
        println!("{}", checksum(input.lines().collect()));
    } else {
        println!("{}", common_letters(input.lines().collect()));
    }
}

fn checksum(input: Vec<&str>) -> i32 {
    let mut t2 = 0;
    let mut t3 = 0;

    let mut map = HashMap::new();
    for id in input {
        map.clear();
        for c in id.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        if map.values().any(|v| *v == 2) {
            t2 += 1;
        }
        if map.values().any(|v| *v == 3) {
            t3 += 1;
        }
    }
    t2 * t3
}

fn common_letters(input: Vec<&str>) -> String {
    for (i, id1) in input.iter().enumerate() {
        for id2 in input.iter().skip(i + 1) {
            match compare(id1, id2) {
                Some(s) => return s,
                None => ()
            }
        }
    }
    String::new()
}

// Return common characters for 2 strings, as long as there is exactly 1 difference
fn compare(a: &str, b: &str) -> Option<String> {
    let mut result = String::new();
    let mut mismatches = false;
    for (c1, c2) in a.chars().zip(b.chars()) {
        if c1 == c2 {
            result.push(c1);
        } else if mismatches {
            return None;
        } else {
            mismatches = true;
        }
    }
    if mismatches { Some(result) } else { None }
}

#[test]
fn test_checksum() {
    assert_eq!(12, checksum(vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"]));
}

#[test]
fn test_common_letters() {
    assert_eq!("fgij", common_letters(vec!["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"]));
}

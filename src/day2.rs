use std::collections::HashMap;

pub fn run(part: i32, input: &str) {
    println!("{}", checksum(input.lines().collect()));
}

fn checksum(input: Vec<&str>) -> i32 {
    let mut t2 = 0;
    let mut t3 = 0;

    for id in input {
        let mut map = HashMap::new();
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

#[test]
fn test_checksum() {
    assert_eq!(12, checksum(vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"]));
}

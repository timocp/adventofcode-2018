pub fn run(part: i32, input: &str) {
    println!("{}", react(input).len());
}

fn react(input: &str) -> String {
    let mut chars = input.as_bytes().to_vec();
    if chars[chars.len() - 1] == 10 {
        chars.pop();
    }
    loop {
        let mut found = None;
        for (i, c) in chars.iter().skip(1).enumerate() {
            if is_pair(*c, chars[i]) {
                found = Some(i);
                break;
            }
        }
        match found {
            Some(i) => {
                chars.remove(i);
                chars.remove(i);
            }
            None => {
                return String::from_utf8(chars).unwrap();
            }
        }
    }
}

fn is_pair(a: u8, b: u8) -> bool {
    a >= 97 && a <= 122 && b == a - 32 || a >= 65 && a <= 90 && b == a + 32
}

#[test]
fn test_react() {
    assert_eq!("", react("aA"));
    assert_eq!("", react("abBA"));
    assert_eq!("abAB", react("abAB"));
    assert_eq!("aabAAB", react("aabAAB"));
    assert_eq!("dabCBAcaDA", react("dabAcCaCBAcCcaDA"));
    assert_eq!(10, react("dabAcCaCBAcCcaDA").len());
    assert_eq!(10, react("dabAcCaCBAcCcaDA\n").len());
}

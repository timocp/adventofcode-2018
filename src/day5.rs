pub fn run(part: i32, input: &str) {
    if part == 1 {
        println!("{}", react(input).len());
    } else {
        println!("{}", shortest_polymer(input).len());
    }
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

fn remove_types(input: &str, uc: char) -> String {
    let lc = uc.to_ascii_lowercase();
    input.chars().filter(|&c| c != uc && c != lc).collect()
}

fn shortest_polymer(input: &str) -> String {
    let mut shortest: Option<String> = None;
    for c in (b'A' .. b'Z').map(|c| c as char) {
        let r = react(&remove_types(input, c));
        shortest = match shortest {
            Some(s) => Some(if r.len() < s.len() { r } else { s }),
            None => Some(r)
        }
    }
    match shortest {
        Some(s) => s,
        None => "".to_string()
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

#[test]
fn test_remove_type() {
    assert_eq!("dbcCCBcCcD", remove_types("dabAcCaCBAcCcaDA", 'A'));
    assert_eq!("daAcCaCAcCcaDA", remove_types("dabAcCaCBAcCcaDA", 'B'));
    assert_eq!("dabAaBAaDA", remove_types("dabAcCaCBAcCcaDA", 'C'));
    assert_eq!("abAcCaCBAcCcaA", remove_types("dabAcCaCBAcCcaDA", 'D'));
}

#[test]
fn test_shortest_polymer() {
    assert_eq!("dbCBcD", react(&remove_types("dabAcCaCBAcCcaDA", 'A')));
    assert_eq!("daCAcaDA", react(&remove_types("dabAcCaCBAcCcaDA", 'B')));
    assert_eq!("daDA", react(&remove_types("dabAcCaCBAcCcaDA", 'C')));
    assert_eq!("abCBAc", react(&remove_types("dabAcCaCBAcCcaDA", 'D')));
    assert_eq!("daDA", shortest_polymer("dabAcCaCBAcCcaDA"));
}

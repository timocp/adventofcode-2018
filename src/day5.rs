use std::collections::VecDeque;
use std::iter::FromIterator;

pub fn run(part: i32, input: &str) {
    if part == 1 {
        println!("{}", react(input).len());
    } else {
        println!("{}", shortest_polymer(input).len());
    }
}

fn react(input: &str) -> String {
    let mut input = VecDeque::from_iter(input.chars());
    if *input.get(input.len() - 1).unwrap() == '\n' {
        input.pop_back();
    }
    let mut output = VecDeque::new();
    output.push_front(input.pop_front().unwrap());
    loop {
        while !input.is_empty() && !output.is_empty() && is_pair(*input.front().unwrap(), *output.back().unwrap()) {
            input.pop_front();
            output.pop_back();
        }
        if input.is_empty() {
            return output.into_iter().collect();
        }
        output.push_back(input.pop_front().unwrap());
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

fn is_pair(a: char, b: char) -> bool {
    a != b && a.to_ascii_uppercase() == b.to_ascii_uppercase()
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

use std::collections::HashSet;

pub fn run(part: i32, input: &str) {
    let input = parse_input(input);
    if part == 1 {
        println!("{}", sum_lines(input));
    } else if part == 2 {
        println!("{}", first_reached_twice(input));
    }
}

fn sum_lines(input: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in input {
        sum += i
    }
    return sum;
}

fn first_reached_twice(input: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut map = HashSet::new();
    map.insert(sum);

    loop {
        for i in &input {
            sum += i;
            if map.contains(&sum) {
                return sum;
            }
            map.insert(sum);
        }
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    let mut vec = vec![];
    let mut lines = input.lines();
    loop {
        match lines.next() {
            Some(s) => {
                match s.parse::<i32>() {
                    Ok(i) => vec.push(i),
                    Err(e) => {
                        eprintln!("Parse error: {}: {}", s, e);
                    }
                }
            },
            None => return vec
        }
    }
}

#[test]
fn test_sum_lines() {
    assert_eq!(3, sum_lines(vec![1, 1, 1]));
    assert_eq!(0, sum_lines(vec![1, 1, -2]));
    assert_eq!(-6, sum_lines(vec![-1, -2, -3]));
}

#[test]
fn test_first_reached_twice() {
    assert_eq!(0, first_reached_twice(vec![1, -1]));
    assert_eq!(10, first_reached_twice(vec![3, 3, 4, -2, -4]));
    assert_eq!(5, first_reached_twice(vec![-6, 3, 8, 5, -6]));
    assert_eq!(14, first_reached_twice(vec![7, 7, -2, -7, -4]));
}

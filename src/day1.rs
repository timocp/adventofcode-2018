pub fn run(part: i32, input: &str) {
    println!("{}", sum_lines(input));
}

fn sum_lines(input: &str) -> i32 {
    let mut sum = 0;
    let mut lines = input.lines();
    loop {
        match lines.next() {
            Some(x) => {
                match x.parse::<i32>() {
                    Ok(n) => sum += n,
                    Err(e) => {
                        eprintln!("Parse error: {}: {}", x, e);
                    }
                }
            },
            None => { break }
        }
    }
    return sum;
}

#[test]
fn test_sum_lines() {
    assert_eq!(3, sum_lines("+1\n+1\n+1\n"));
    assert_eq!(0, sum_lines("+1\n+1\n-2\n"));
    assert_eq!(-6, sum_lines("-1\n-2\n-3\n"));
}

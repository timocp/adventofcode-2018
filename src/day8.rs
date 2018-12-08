use super::{Part,Part::*};

pub fn run(part: Part, input: &str) {
    match part {
        One => println!("{}", metadata_sum(&parse_input(input))),
        Two => println!()
    }
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split(" ").filter_map(|s| s.trim().parse().ok()).collect()
}

fn metadata_sum(input: &[usize]) -> usize {
    metadata_sum_child(input).0
}

// returns (sum, size) indicating the metadata sum of node starting at `start`
// and how many numbers were used up
fn metadata_sum_child(input: &[usize]) -> (usize, usize) {
    let children = input[0];
    let metadata = input[1];
    let mut index = 2;
    let mut sum = 0;
    for _ in 0..children {
        let (child_sum, child_used) = metadata_sum_child(&input[index..]);
        sum += child_sum;
        index += child_used;
    }
    for i in index..index + metadata {
        sum += input[i];
    }
    (sum, index + metadata)
}

#[cfg(test)]
mod tests {
    fn test_input() -> &'static str {
        "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2\n"
    }

    #[test]
    fn test_metadata_sum() {
        assert_eq!(138, super::metadata_sum(&super::parse_input(test_input())));
    }
}

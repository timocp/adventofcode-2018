use super::{Part, Part::*};

pub fn run(part: Part, input: &str) {
    let input = parse_input(input);
    println!(
        "{}",
        match part {
            One => metadata_sum(&input),
            Two => value(&input),
        }
    );
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(" ")
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

fn metadata_sum(input: &[usize]) -> usize {
    parse_tree(input).sum
}

fn value(input: &[usize]) -> usize {
    parse_tree(input).value
}

struct Info {
    used: usize,
    sum: usize,
    value: usize,
}

fn parse_tree(input: &[usize]) -> Info {
    let num_children = input[0];
    let num_metadata = input[1];
    let mut info = Info {
        used: 2,
        sum: 0,
        value: 0,
    };
    let mut children = vec![];

    for _ in 0..num_children {
        let child = parse_tree(&input[info.used..]);
        info.sum += child.sum;
        info.used += child.used;
        children.push(child);
    }

    for i in input.iter().skip(info.used).take(num_metadata) {
        info.sum += i;
        info.used += 1;
        if num_children == 0 {
            info.value += i;
        } else if i - 1 < children.len() {
            info.value += children[i - 1].value;
        }
    }

    info
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2\n"
    }

    #[test]
    fn test_metadata_sum() {
        assert_eq!(138, metadata_sum(&parse_input(test_input())));
    }

    #[test]
    fn test_value() {
        assert_eq!(66, value(&parse_input(test_input())));
    }
}

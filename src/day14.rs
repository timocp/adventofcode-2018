use super::{Part, Part::*};

pub fn run(part: Part, input: &str) {
    let input = parse_input(input);
    match part {
        One => println!("{}", part1(input)),
        Two => println!(),
    }
}

fn parse_input(input: &str) -> usize {
    input.trim().parse().unwrap()
}

fn starting_recipes() -> Vec<u8> {
    vec![3, 7]
}

fn part1(target: usize) -> String {
    let mut recipes = starting_recipes();
    let mut elves: Vec<usize> = vec![0, 1];
    while recipes.len() < target + 10 {
        let new = recipes[elves[0]] + recipes[elves[1]];
        if new >= 10 {
            recipes.push(new / 10);
        }
        recipes.push(new % 10);
        for elf in &mut elves {
            *elf = (*elf + recipes[*elf] as usize + 1) % recipes.len();
        }
    }
    recipes
        .iter()
        .skip(target)
        .take(10)
        .map(|r| (r + 48) as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!("5158916779", part1(9));
        assert_eq!("0124515891", part1(5));
        assert_eq!("9251071085", part1(18));
        assert_eq!("5941429882", part1(2018));
    }
}

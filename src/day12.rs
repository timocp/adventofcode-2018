use super::{Part, Part::*};
use std::collections::VecDeque;

pub fn run(part: Part, input: &str) {
    match part {
        One => println!("{}", part1(input, 20)),
        Two => println!(),
    }
}

fn part1(input: &str, generations: i32) -> i32 {
    let (rules, mut s1) = parse_input(input);
    let mut s2 = State {
        plants: VecDeque::new(),
        offset: 0,
    };
    let mut count = 0;
    loop {
        grow(rules, &s1, &mut s2);
        count += 1;
        if count == generations {
            return s2.sum_pots();
        }
        grow(rules, &s2, &mut s1);
        count += 1;
        if count == generations {
            return s1.sum_pots();
        }
    }
}

fn parse_input(input: &str) -> (u32, State) {
    let mut rules = 0;
    let mut state = State {
        plants: VecDeque::new(),
        offset: 0,
    };
    for line in input.lines() {
        if line.starts_with("initial state: ") {
            for c in line.chars().skip(15) {
                if c == '#' {
                    state.plants.push_back(true);
                } else if c == '.' {
                    state.plants.push_back(false);
                } else {
                    break;
                }
            }
        } else if line.ends_with("#") {
            let mut v = 0;
            for (i, c) in line.chars().take(5).enumerate() {
                if c == '#' {
                    v += 2u32.pow(4 - i as u32);
                }
            }
            rules |= 2u32.pow(v - 1);
        }
    }
    state.trim();
    (rules, state)
}

#[derive(Debug)]
struct State {
    plants: VecDeque<bool>,
    offset: i32,
}

impl State {
    fn trim(&mut self) {
        while self.plants.front() == Some(&false) {
            self.plants.pop_front();
            self.offset += 1;
        }
        while self.plants.back() == Some(&false) {
            self.plants.pop_back();
        }
    }

    fn expand(&mut self, other: &State) {
        while self.offset + 4 > other.offset {
            self.plants.push_front(false);
            self.offset -= 1;
        }
        while self.end() < other.end() + 4 {
            self.plants.push_back(false);
        }
    }

    // earliest index you can ask for
    fn start(&self) -> i32 {
        self.offset
    }

    // last index you can ask for
    fn end(&self) -> i32 {
        self.plants.len() as i32 + self.offset - 1
    }

    fn get(&self, index: i32) -> bool {
        if index < self.offset || index > self.end() {
            false
        } else {
            self.plants[(index - self.offset) as usize]
        }
    }

    fn set(&mut self, index: i32, b: bool) {
        self.plants[(index - self.offset) as usize] = b;
    }

    fn sum_pots(&self) -> i32 {
        let mut sum = 0;
        for i in self.start()..=self.end() {
            if self.get(i) {
                sum += i;
            }
        }
        sum
    }
}

fn grow(rules: u32, s1: &State, s2: &mut State) {
    s2.expand(&s1);

    let mut v = 0u32;
    for i in s2.start()..=s2.end() {
        v = (v << 1) & 31;
        if s1.get(i) {
            v |= 1;
        }
        if i - 2 < s2.start() || i - 2 > s2.end() {
            continue;
        }
        if v > 0 && (rules >> (v - 1)) & 1 == 1 {
            s2.set(i - 2, true);
        } else {
            s2.set(i - 2, false);
        }
    }

    s2.trim();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    fn test_input() -> &'static str {
        "initial state: #..#.#..##......###...###

..... => .
...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #\n"
    }

    #[test]
    fn test_parse_input() {
        let (rules, state) = parse_input(test_input());
        assert_eq!(
            VecDeque::from_iter(vec![
                true, false, false, true, false, true, false, false, true, true, false, false,
                false, false, false, false, true, true, true, false, false, false, true, true,
                true
            ]),
            state.plants,
        );
        assert_eq!(0, state.offset);
        assert_eq!(1045450380, rules);
        assert_eq!(true, state.get(0));
        assert_eq!(false, state.get(1));
        assert_eq!(false, state.get(2));
        assert_eq!(true, state.get(3));
    }

    #[test]
    fn test_grow() {
        let (rules, mut state) = parse_input(test_input());
        let mut state2 = State {
            plants: VecDeque::new(),
            offset: 0,
        };
        grow(rules, &state, &mut state2);
        assert_eq!(
            VecDeque::from_iter(vec![
                true, false, false, false, true, false, false, false, false, true, false, false,
                false, false, false, true, false, false, true, false, false, true, false, false,
                true
            ]),
            state2.plants
        );
        assert_eq!(0, state2.offset);
        grow(rules, &state2, &mut state);
        assert_eq!(
            VecDeque::from_iter(vec![
                true, true, false, false, true, true, false, false, false, true, true, false,
                false, false, false, true, false, false, true, false, false, true, false, false,
                true, true
            ]),
            state.plants
        );
        assert_eq!(0, state2.offset);
        grow(rules, &state, &mut state2);
        assert_eq!(
            VecDeque::from_iter(vec![
                true, false, true, false, false, false, true, false, false, true, false, true,
                false, false, false, false, true, false, false, true, false, false, true, false,
                false, false, true
            ]),
            state2.plants
        );
        assert_eq!(-1, state2.offset);
    }

    #[test]
    fn test_part1() {
        assert_eq!(325, part1(test_input(), 20));
    }
}

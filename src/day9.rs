use std::collections::VecDeque;
use super::{Part,Part::*};

pub fn run(part: Part, input: &str) {
    let input = parse_input(input);
    match part {
        One => println!("{}", simulate_game(input[0], input[1])),
        Two => println!("{}", simulate_game(input[0], input[1])),
    }
}

fn simulate_game(players: usize, last_marble: usize) -> usize {
    let mut game: VecDeque<usize> = VecDeque::with_capacity(last_marble);
    let mut scores = vec![0; players as usize];
    let mut current = 0;
    game.push_front(0);
    for marble in 1..last_marble+1 {
        let player = (marble - 1) % players;
        if marble % 23 == 0 {
            scores[player] += marble;
            current = if current >= 7 { current - 7 } else { game.len() - (7 - current) };
            scores[player] += game[current];
            game.remove(current);
        } else {
            current = (current + 2) % game.len();
            if current == 0 {
                game.push_back(marble);
                current = game.len() - 1;
            } else {
                game.insert(current, marble);
            }
        }
    }
    *scores.iter().max().unwrap()
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split_whitespace().filter_map(|s| s.parse().ok()).collect()
}

#[test]
fn test_simulate_game() {
    assert_eq!(32, simulate_game(9, 25));
    assert_eq!(8317, simulate_game(10, 1618));
    assert_eq!(146373, simulate_game(13, 7999));
    assert_eq!(2764, simulate_game(17, 1104));
    assert_eq!(54718, simulate_game(21, 6111));
    assert_eq!(37305, simulate_game(30, 5807));
}

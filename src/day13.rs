use super::{Part, Part::*};
use std::fmt;
use std::fmt::Write;

pub fn run(part: Part, input: &str) {
    let mut mine = parse_input(input);
    match part {
        One => println!("{:?}", mine.first_crash()),
        Two => println!(),
    }
}

#[derive(Clone)]
enum Cell {
    Empty,
    Corner1,
    Corner2,
    NS,
    EW,
    Intersection,
}

impl Cell {
    fn to_char(&self) -> char {
        match &self {
            Cell::Empty => ' ',
            Cell::Corner1 => '/',
            Cell::Corner2 => '\\',
            Cell::NS => '|',
            Cell::EW => '-',
            Cell::Intersection => '+',
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_char(c: char) -> Result<Direction, String> {
        match c {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            _ => Err(format!("Invalid character {}", c)),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
struct Cart {
    x: usize,
    y: usize,
    facing: Direction,
    next_turn: Turn,
}

impl Cart {
    fn intersection_turn(&mut self) {
        self.facing = match self.next_turn {
            Turn::Left => match self.facing {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
            Turn::Right => match self.facing {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            Turn::Straight => self.facing,
        };
        self.next_turn = match self.next_turn {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }

    fn move_turn(&mut self, map: &Vec<Vec<Cell>>) {
        match self.facing {
            Direction::North => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
        }

        match map[self.y][self.x] {
            Cell::Corner1 => match self.facing {
                Direction::North => self.facing = Direction::East,
                Direction::East => self.facing = Direction::North,
                Direction::South => self.facing = Direction::West,
                Direction::West => self.facing = Direction::South,
            },
            Cell::Corner2 => match self.facing {
                Direction::North => self.facing = Direction::West,
                Direction::East => self.facing = Direction::South,
                Direction::South => self.facing = Direction::East,
                Direction::West => self.facing = Direction::North,
            },
            Cell::Intersection => self.intersection_turn(),
            Cell::Empty | Cell::NS | Cell::EW => (),
        }
    }
}

struct Mine {
    map: Vec<Vec<Cell>>,
    carts: Vec<Cart>, // kept in order
}

impl fmt::Debug for Mine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0;
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if i < self.carts.len() && self.carts[i].y == y && self.carts[i].x == x {
                    f.write_char(self.carts[i].facing.to_char()).unwrap();
                    i += 1;
                } else {
                    f.write_char(cell.to_char()).unwrap();
                }
            }
            f.write_char('\n').unwrap();
        }
        Ok(())
    }
}

impl Mine {
    fn first_crash(&mut self) -> Position {
        loop {
            for cart_id in 0..self.carts.len() {
                self.carts[cart_id].move_turn(&self.map);

                // has it crashed?
                for (cart2_id, cart2) in self.carts.iter().enumerate() {
                    if cart2_id != cart_id
                        && self.carts[cart_id].x == cart2.x
                        && self.carts[cart_id].y == cart2.y
                    {
                        return Position {
                            x: cart2.x,
                            y: cart2.y,
                        };
                    }
                }
            }
            self.carts.sort_by_key(|c| (c.y, c.x));
        }
    }
}

fn parse_input(input: &str) -> Mine {
    let mut mine = Mine {
        map: vec![],
        carts: vec![],
    };
    for (y, line) in input.lines().enumerate() {
        mine.map.push(vec![Cell::Empty; line.len()]);
        for (x, c) in line.chars().enumerate() {
            if c == ' ' {
                mine.map[y][x] = Cell::Empty;
            } else if c == '/' {
                mine.map[y][x] = Cell::Corner1;
            } else if c == '\\' {
                mine.map[y][x] = Cell::Corner2;
            } else if c == '^' || c == 'v' || c == '<' || c == '>' {
                if c == '^' || c == 'v' {
                    mine.map[y][x] = Cell::NS;
                } else {
                    mine.map[y][x] = Cell::EW;
                }
                mine.carts.push(Cart {
                    x: x,
                    y: y,
                    facing: Direction::from_char(c).unwrap(),
                    next_turn: Turn::Left,
                });
            } else if c == '-' {
                mine.map[y][x] = Cell::EW;
            } else if c == '|' {
                mine.map[y][x] = Cell::NS;
            } else if c == '+' {
                mine.map[y][x] = Cell::Intersection;
            }
        }
    }
    mine
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        "/->-\\        \n|   |  /----\\\n| /-+--+-\\  |\n| | |  | v  |\n\\-+-/  \\-+--/\n  \\------/   \n"
    }

    #[test]
    fn test_parse_input() {
        let mine = parse_input(test_input());
        assert_eq!(format!("{:?}", mine), test_input());
    }

    #[test]
    fn test_first_crash() {
        let mut mine = parse_input(test_input());
        assert_eq!(Position { x: 7, y: 3 }, mine.first_crash());
    }
}

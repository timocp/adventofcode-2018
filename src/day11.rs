use super::{Part, Part::*};
use std::cmp;
use std::fmt;

pub fn run(part: Part, input: &str) {
    let grid_serial: usize = input.trim().parse().unwrap();
    match part {
        One => println!("{}", largest_power(grid_serial)),
        Two => println!("{}", max_powers(grid_serial)),
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Answer {
    x: usize,
    y: usize,
    size: usize,
    value: i32,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.size)
    }
}

// returns (x,y) of the top-left of the largest 3x3 grid
fn largest_power(grid_serial: usize) -> Answer {
    let grid = init_grid(grid_serial);
    let mut max = Answer {
        x: 0,
        y: 0,
        size: 3,
        value: -6 * (3 * 3) as i32,
    };
    for x in 0..=(300 - 3) {
        for y in 0..=(300 - 3) {
            let mut total = 0;
            for xx in 0..3 {
                for yy in 0..3 {
                    total += grid[x + xx][y + yy];
                }
            }
            if total > max.value {
                max.value = total;
                max.x = x + 1;
                max.y = y + 1;
            }
        }
    }
    max
}

fn max_powers(grid_serial: usize) -> Answer {
    let grid = init_grid(grid_serial);
    let sums = summed_area(&grid);
    let mut answer = Answer {
        x: 0,
        y: 0,
        size: 0,
        value: 300 * 300 * -5,
    };

    for x in 0..299 {
        for y in 0..299 {
            for d in 1..(300 - cmp::max(x, y) - 1) {
                let total = sums[x + d][y + d] + sums[x][y] - sums[x + d][y] - sums[x][y + d];
                if total > answer.value {
                    answer.x = x + 2;
                    answer.y = y + 2;
                    answer.size = d;
                    answer.value = total;
                }
            }
        }
    }
    answer
}

fn summed_area(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut sums = vec![vec![0; 300]; 300];
    for x in 0..300 {
        for y in 0..300 {
            if x == 0 && y == 0 {
                sums[x][y] = grid[x][y];
            } else if x == 0 {
                sums[x][y] = grid[x][y] + sums[x][y - 1];
            } else if y == 0 {
                sums[x][y] = grid[x][y] + sums[x - 1][y];
            } else {
                sums[x][y] = grid[x][y] + sums[x][y - 1] + sums[x - 1][y] - sums[x - 1][y - 1];
            }
        }
    }
    sums
}

fn init_grid(grid_serial: usize) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; 300]; 300];
    for x in 0..300 {
        for y in 0..300 {
            grid[x][y] = power_level(x + 1, y + 1, grid_serial);
        }
    }
    grid
}

fn power_level(x: usize, y: usize, grid_serial: usize) -> i32 {
    let rack_id = x + 10;
    let power = (rack_id * y + grid_serial) * rack_id;
    (if power >= 100 { power / 100 % 10 } else { 0 } as i32 - 5)
}

#[test]
fn test_power_level() {
    assert_eq!(4, power_level(3, 5, 8));
    assert_eq!(-5, power_level(122, 79, 57));
    assert_eq!(0, power_level(217, 196, 39));
    assert_eq!(4, power_level(101, 153, 71));
}

#[test]
fn test_largest_power() {
    assert_eq!(
        Answer {
            x: 33,
            y: 45,
            size: 3,
            value: 29
        },
        largest_power(18)
    );
    assert_eq!(
        Answer {
            x: 21,
            y: 61,
            size: 3,
            value: 30
        },
        largest_power(42)
    );
}

#[ignore]
#[test]
fn test_max_powers() {
    assert_eq!(
        Answer {
            x: 90,
            y: 269,
            size: 16,
            value: 113
        },
        max_powers(18)
    );
    assert_eq!(
        Answer {
            x: 232,
            y: 251,
            size: 12,
            value: 119
        },
        max_powers(42)
    );
}

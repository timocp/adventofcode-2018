use super::{Part, Part::*};

pub fn run(part: Part, input: &str) {
    let grid_serial: usize = input.trim().parse().unwrap();
    match part {
        One => println!("{:?}", largest_power(grid_serial)),
        Two => println!(),
    }
}

// returns (x,y) of the top-left of the largest 3x3 grid
fn largest_power(grid_serial: usize) -> (usize, usize) {
    let grid = init_grid(grid_serial);
    let mut max_value = -46;
    let mut max_position = (0, 0);
    for x in 0..298 {
        for y in 0..298 {
            let mut total = 0;
            for xx in 0..3 {
                for yy in 0..3 {
                    total += grid[(x + xx) * 300 + y + yy];
                }
            }
            if total > max_value {
                max_value = total;
                max_position = (x + 1, y + 1);
            }
        }
    }
    max_position
}

fn init_grid(grid_serial: usize) -> Vec<i32> {
    let mut grid = vec![0; 300 * 300];
    for x in 0..300 {
        for y in 0..300 {
            grid[x * 300 + y] = power_level(x + 1, y + 1, grid_serial);
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
    assert_eq!((33, 45), largest_power(18));
    assert_eq!((21, 61), largest_power(42));
}

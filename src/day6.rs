use super::{Part,Part::*};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn run(part: Part, input: &str) {
    match part {
        One => println!("{}", largest_finite_area(&parse_input(input))),
        Two => println!("{}", safe_area(&parse_input(input), 10000))
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Closest {
    distance: usize,
    coord: Option<usize>
}

fn largest_finite_area(coords: &Vec<Point>) -> usize {
    let (min, max) = box_size(coords);

    let mut closest: HashMap<Point, Closest> = HashMap::new();
    for (c, coord) in coords.iter().enumerate() {
        for x in min.x..max.x+1 {
            for y in min.y..max.y+1 {
                let distance = ((coord.x - x).abs() + (coord.y - y).abs()) as usize;
                match closest.entry(Point{x: x, y: y}) {
                    Entry::Occupied(ent) => {
                        let ent = ent.into_mut();
                        if distance < ent.distance {
                            ent.distance = distance;
                            ent.coord = Some(c);
                        } else if distance == ent.distance {
                            ent.coord = None;
                        }
                    }
                    Entry::Vacant(ent) => {
                        ent.insert(Closest{distance: distance, coord: Some(c)});
                    }
                }
            }
        }
    }

    // areas, Some(x) means finite area x, None means infinite
    let mut areas: Vec<Option<usize>> = vec![Some(0); coords.len()];
    for y in min.y..max.y+1 {
        for x in min.x..max.x+1 {
            let closest = closest.get(&Point{x: x, y: y}).unwrap();
            if let Some(c) = closest.coord {
                if y == min.y || y == max.y || x == min.x || x == max.x {
                    areas[c] = None;
                } else if let Some(v) = areas[c] {
                    areas[c] = Some(v + 1);
                }
            }
        }
    }
    areas.into_iter().filter_map(|area| area).max().unwrap()
}

fn safe_area(coords: &Vec<Point>, limit: usize) -> usize {
    let (min, max) = box_size(coords);
    let mut area = 0;
    for x in min.x..max.x+1 {
        'cell: for y in min.y..max.y+1 {
            let mut distance = 0;
            for coord in coords.iter() {
                distance += ((coord.x - x).abs() + (coord.y - y).abs()) as usize;
                if distance >= limit {
                    continue 'cell;
                }
            }
            area += 1;
        }
    }
    area
}

fn box_size(coords: &Vec<Point>) -> (Point, Point) {
    let mut min = *coords.first().unwrap();
    let mut max = *coords.first().unwrap();
    for c in coords.iter().skip(1) {
        if c.x < min.x {
            min.x = c.x;
        } else if c.x > max.x {
            max.x = c.x;
        }
        if c.y < min.y {
            min.y = c.y;
        } else if c.y > max.y {
            max.y = c.y;
        }
    }
    (min, max)
}

fn parse_input(input: &str) -> Vec<Point> {
    let mut list = vec![];
    for line in input.lines() {
        let coords: Vec<_> = line.split(", ").filter_map(|s| s.parse().ok()).collect();
        list.push(Point{x: coords[0], y: coords[1]});
    }
    list
}

#[test]
fn test_run() {
    let test_input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
    assert_eq!(17, largest_finite_area(&parse_input(test_input)));
    assert_eq!(16, safe_area(&parse_input(test_input), 32));
}

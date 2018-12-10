use image::ImageBuffer;
use regex::Regex;
use super::{Part,Part::*};

pub fn run(part: Part, input: &str) {
    match part {
        One => {
            find_message(&mut parse_input(input), true);
            println!("Wrote day10.png containing answer.");
        }
        Two => println!("{}", find_message(&mut parse_input(input), false))
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64
}

#[derive(Clone, Copy)]
struct Star {
    point: Point,
    dx: i64,
    dy: i64
}

fn find_message(stars: &mut Vec<Star>, write_image: bool) -> i32 {
    let mut s = 0;
    let mut last_area = area(stars);
    loop {
        s += 1;
        tick_forward(stars);
        let a = area(stars);
        if a > last_area {
            s -= 1;
            tick_backwards(stars);
            break;
        }
        last_area = a;
    }
    if write_image {
        to_image(stars);
    }
    s
}

fn tick_forward(stars: &mut Vec<Star>) {
    for star in stars.iter_mut() {
        star.point.x += star.dx;
        star.point.y += star.dy;
    }
}

fn tick_backwards(stars: &mut Vec<Star>) {
    for star in stars.iter_mut() {
        star.point.x -= star.dx;
        star.point.y -= star.dy;
    }
}

fn to_image(stars: &Vec<Star>) {
    let (min, max) = boxsize(stars);
    let width = max.x - min.x + 3; // 2 is border
    let height = max.y - min.y + 3;
    let mut img = ImageBuffer::from_fn(width as u32, height as u32, |_x, _y| image::Luma([0u8]));
    for star in stars.iter() {
        let x = star.point.x - min.x + 1;
        let y = star.point.y - min.y + 1;
        img.put_pixel(x as u32, y as u32, image::Luma([255u8]));
    }
    img.save("day10.png").unwrap();
}

fn area(stars: &Vec<Star>) -> u64 {
    let (min, max) = boxsize(stars);
    ((max.x - min.x + 1).abs() * (max.y - min.y + 1).abs()) as u64
}

fn boxsize(stars: &Vec<Star>) -> (Point, Point) {
    let mut min = stars.first().unwrap().point;
    let mut max = stars.first().unwrap().point;
    for star in stars.iter().skip(1) {
        if star.point.x < min.x {
            min.x = star.point.x;
        } else if star.point.x > max.x {
            max.x = star.point.x;
        }
        if star.point.y < min.y {
            min.y = star.point.y;
        } else if star.point.y > max.y {
            max.y = star.point.y;
        }
    }
    (min, max)
}

fn parse_input(input: &str) -> Vec<Star> {
    let re = Regex::new(r"^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$").unwrap();
    let mut stars = vec![];
    for line in input.lines() {
        match re.captures(line) {
            Some(cap) => stars.push(Star{
                            point: Point{
                                x: cap[1].parse().unwrap(),
                                y: cap[2].parse().unwrap()
                            },
                            dx: cap[3].parse().unwrap(),
                            dy: cap[4].parse().unwrap()
                        }),
            None => eprintln!("parse error: {}", line)
        }
    }
    stars
}

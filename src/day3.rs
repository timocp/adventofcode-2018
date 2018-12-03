use regex::Regex;

pub fn run(part: i32, input: &str) {
    let claims = parse_input(input);
    if part == 1 {
        println!("{}", overlapping_squares(claims));
    }
}

fn overlapping_squares(claims: Vec<Claim>) -> i32 {
    let mut fabric = [[0; 1000]; 1000];
    let mut count = 0;
    for claim in claims {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                fabric[x][y] += 1;
                if fabric[x][y] == 2 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn parse_input(input: &str) -> Vec<Claim> {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let mut claims = vec![];
    for line in input.lines() {
        match re.captures(line) {
            Some(cap) => 
                claims.push(Claim{
                    id: cap[1].parse().unwrap(),
                    left: cap[2].parse().unwrap(),
                    top: cap[3].parse().unwrap(),
                    width: cap[4].parse().unwrap(),
                    height: cap[5].parse().unwrap(),
                }),
            None => eprintln!("parse error: {}", line)
        }
    }
    claims
}

#[derive(Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize
}

#[test]
fn test_run() {
}

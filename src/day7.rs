use petgraph::Incoming;
use petgraph::graphmap::DiGraphMap;
use super::{Part,Part::*};

pub fn run(part: Part, input: &str) {
    match part {
        One => println!("{}", get_order(parse_input(input))),
        Two => println!("{}", time_simulation(parse_input(input), 5, 60))
    }
}

fn get_order(mut g: DiGraphMap<char, ()>) -> String {
    let mut output = "".to_string();
    while let Some(next) = next_step(&g) {
        output.push(next);
        g.remove_node(next);
    }
    output
}

fn time_simulation(mut g: DiGraphMap<char, ()>, num_workers: usize, base_time: usize) -> i32 {
    let mut seconds = 0;
    let mut workers: Vec<Option<(char, usize)>> = vec![None; num_workers];
    let mut in_progress: Vec<char> = workers.iter().filter_map(|worker| *worker).map(|job| job.0).collect();
    loop {
        // assign jobs to idle workers
        let mut running = false;
        for worker in workers.iter_mut().filter(|worker| worker.is_none()) {
            if let Some(step) = next_step_except(&g, &in_progress) {
                *worker = Some((step, base_time + (step as usize - 65)));
                in_progress.push(step);
                running = true;
            }
        }

        // reduce jobs in progress
        for worker in workers.iter_mut() {
            if let Some(job) = worker {
                if job.1 == 0 {
                    g.remove_node(job.0);
                    in_progress.retain(|&x| x != job.0);
                    *worker = None;
                    running = true;
                } else {
                    job.1 -= 1;
                    running = true;
                }
            }
        }
        if !running {
            return seconds;
        }

        seconds += 1
    }
}

fn next_step(g: &DiGraphMap<char, ()>) -> Option<char> {
    next_step_except(g, &vec![])
}

fn next_step_except(g: &DiGraphMap<char, ()>, except: &Vec<char>) -> Option<char> {
    let mut next: Vec<_> = g.nodes()
        .filter(|n| g.neighbors_directed(*n, Incoming).count() == 0)
        .filter(|n| !except.contains(n))
        .collect();
    if next.is_empty() {
        None
    } else {
        next.sort();
        Some(*next.first().unwrap())
    }
}

fn parse_input(input: &str) -> DiGraphMap<char, ()> {
    let mut g = DiGraphMap::new();
    for line in input.lines() {
        let words: Vec<char> = line.chars().collect();
        let from = words[5];
        let to = words[36];
        g.add_edge(from, to, ());
    }
    g
}

#[test]
fn test_run() {
    let test_input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
    assert_eq!("CABDFE", get_order(parse_input(test_input)));
    assert_eq!(15, time_simulation(parse_input(test_input), 2, 0));
}

use petgraph::Incoming;
use petgraph::graphmap::DiGraphMap;

pub fn run(part: i32, input: &str) {
    println!("{}", get_order(parse_input(input)));
}

fn get_order(mut g: DiGraphMap<char, ()>) -> String {
    let mut output = "".to_string();
    while g.node_count() > 0 {
        let mut next: Vec<_> = g.nodes().filter(|n| g.neighbors_directed(*n, Incoming).count() == 0).collect();
        next.sort();
        let next = *next.first().unwrap();
        output.push(next);
        g.remove_node(next);
    }
    output
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
fn test_get_order() {
    let test_input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
    assert_eq!("CABDFE", get_order(parse_input(test_input)));
}

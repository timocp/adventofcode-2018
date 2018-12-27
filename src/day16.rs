use self::Op::*;
use super::{Part, Part::*};
use std::collections::HashSet;
use std::slice::Iter;

pub fn run(part: Part, input: &str) {
    let (samples, program) = parse_input(input);
    match part {
        One => println!(
            "{}",
            samples
                .iter()
                .filter(|sample| sample.probe().len() >= 3)
                .count()
        ),
        Two => {
            let mut vm = Device::new();
            vm.run_program(&reverse_engineer(&samples), &program);
            println!("{}", vm.reg[0]);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Op {
    fn each() -> Iter<'static, Op> {
        static OPCODES: [Op; 16] = [
            Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir,
            Eqri, Eqrr,
        ];
        OPCODES.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Device {
    reg: [usize; 4],
}

impl Device {
    fn new() -> Device {
        Device { reg: [0, 0, 0, 0] }
    }

    fn run_program(&mut self, map: &Vec<Op>, program: &Vec<Inst>) {
        for inst in program.iter() {
            self.exec(map[inst.opcode], inst.a, inst.b, inst.c);
        }
    }

    #[rustfmt::skip]
    fn exec(&mut self, op: Op, av: usize, bv: usize, c: usize) {
        let ar = self.reg[av];
        let br = self.reg[bv];
        self.reg[c] = match op {
            Addr => ar + br,
            Addi => ar + bv,
            Mulr => ar * br,
            Muli => ar * bv,
            Banr => ar & br,
            Bani => ar & bv,
            Borr => ar | br,
            Bori => ar | bv,
            Setr => ar,
            Seti => av,
            Gtir => if av > br { 1 } else { 0 },
            Gtri => if ar > bv { 1 } else { 0 },
            Gtrr => if ar > br { 1 } else { 0 },
            Eqir => if av == br { 1 } else { 0 },
            Eqri => if ar == bv { 1 } else { 0 },
            Eqrr => if ar == br { 1 } else { 0 },
        }
    }
}

#[derive(Debug, Clone)]
struct Sample {
    before: Device,
    instr: [usize; 4],
    after: Device,
}

impl Sample {
    fn new() -> Sample {
        Sample {
            before: Device::new(),
            instr: [0, 0, 0, 0],
            after: Device::new(),
        }
    }

    // return set of opcodes which this sample could match
    fn probe(&self) -> HashSet<Op> {
        let mut results = HashSet::new();
        for op in Op::each() {
            let mut vm = self.before.clone();
            vm.exec(*op, self.instr[1], self.instr[2], self.instr[3] as usize);
            if vm == self.after {
                results.insert(*op);
            }
        }
        results
    }
}

fn reverse_engineer(samples: &Vec<Sample>) -> Vec<Op> {
    // an array of sets of possible matching opcodes, indexed by the input opcode
    let mut maybe: Vec<HashSet<Op>> = vec![HashSet::new(); 16];
    for sample in samples {
        let opcode = sample.instr[0];
        let couldbe = sample.probe();
        if maybe[opcode].is_empty() {
            maybe[opcode] = couldbe;
        } else {
            // in-place intersect
            maybe[opcode].retain(|op| couldbe.contains(op));
        }
    }

    // some opcodes are now know.  remove them from the possibilities of others.
    // repeat until no unknowns.
    let mut done = false;
    let mut elim: Vec<bool> = vec![false; 16];
    while !done {
        done = true;
        for i in 0..maybe.len() {
            if maybe[i].len() == 1 && !elim[i] {
                elim[i] = true;
                let op = maybe[i].iter().nth(0).unwrap().clone();
                for j in 0..maybe.len() {
                    if i != j {
                        if maybe[j].remove(&op) {
                            done = false;
                        }
                    }
                }
            }
        }
    }

    // each maybe is now a set of 1
    maybe
        .iter()
        .map(|set| *set.iter().nth(0).unwrap())
        .collect()
}

#[derive(Debug)]
struct Inst {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

fn parse_input(input: &str) -> (Vec<Sample>, Vec<Inst>) {
    let mut samples = vec![];
    let mut sample = Sample::new();
    let mut in_samples = true;
    let mut program = vec![];
    for line in input.lines() {
        if line.starts_with("Before: [") {
            in_samples = true;
            let s: String = line.chars().skip(9).take_while(|&c| c != ']').collect();
            for (i, n) in parse_numbers(&s, ", ").into_iter().enumerate() {
                sample.before.reg[i] = n;
            }
        } else if line.starts_with("After:  [") {
            let s: String = line.chars().skip(9).take_while(|&c| c != ']').collect();
            for (i, n) in parse_numbers(&s, ", ").into_iter().enumerate() {
                sample.after.reg[i] = n;
            }
            samples.push(sample.clone());
            in_samples = false;
        } else if line.len() > 0 {
            let numbers = parse_numbers(&line, " ");
            if in_samples {
                for (i, n) in numbers.into_iter().enumerate() {
                    sample.instr[i] = n;
                }
            } else {
                program.push(Inst {
                    opcode: numbers[0],
                    a: numbers[1],
                    b: numbers[2],
                    c: numbers[3],
                });
            }
        }
    }
    (samples, program)
}

fn parse_numbers(input: &str, delim: &str) -> Vec<usize> {
    input.split(delim).map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        "Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]\n\n"
    }

    #[test]
    fn test_parse_input() {
        let (samples, _program) = parse_input(test_input());
        assert_eq!(1, samples.len());
    }

    #[test]
    fn test_probe() {
        let (samples, _program) = parse_input(test_input());
        let mut expected = HashSet::new();
        expected.insert(Addi);
        expected.insert(Mulr);
        expected.insert(Seti);
        assert_eq!(expected, samples[0].probe());
    }
}

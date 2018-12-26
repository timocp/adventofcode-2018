use self::Op::*;
use super::{Part, Part::*};
use std::slice::Iter;

pub fn run(part: Part, input: &str) {
    let (samples, _program) = parse_input(input);
    match part {
        One => println!(
            "{}",
            samples
                .iter()
                .filter(|sample| sample.probe().len() >= 3)
                .count()
        ),
        Two => println!(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

    // return list of opcodes which this sample could match
    fn probe(&self) -> Vec<Op> {
        let mut results = vec![];
        for op in Op::each() {
            let mut vm = self.before.clone();
            vm.exec(*op, self.instr[1], self.instr[2], self.instr[3] as usize);
            if vm == self.after {
                results.push(*op);
            }
        }
        results
    }
}

// placeholder for part 2
struct Program {}

fn parse_input(input: &str) -> (Vec<Sample>, Program) {
    let mut samples = vec![];
    let mut sample = Sample::new();
    let mut in_samples = true;
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
            for (i, n) in parse_numbers(&line, " ").into_iter().enumerate() {
                if in_samples {
                    sample.instr[i] = n;
                }
            }
        }
    }
    (samples, Program {})
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
        assert_eq!(vec![Addi, Mulr, Seti], samples[0].probe());
    }
}

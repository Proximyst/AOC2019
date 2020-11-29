use crate::intcode::*;

pub struct DiagnosticIoBus(isize, isize);

impl IoBus for DiagnosticIoBus {
    fn input(&mut self) -> Option<isize> {
        Some(self.1)
    }

    fn output(&mut self, i: isize) -> bool {
        self.0 = i;
        false
    }
}

#[aoc_generator(day5)]
pub fn gen(input: &str) -> Vec<isize> {
    input.split(',').flat_map(|i| i.parse()).collect()
}

#[aoc(day5, part1)]
pub fn part1_impl1(input: &Vec<isize>) -> isize {
    let mut bus = DiagnosticIoBus(0, 1);
    run(&mut input.clone(), (0, 0), &mut bus);
    bus.0
}

#[aoc(day5, part2)]
pub fn part2_impl1(input: &Vec<isize>) -> isize {
    let mut bus = DiagnosticIoBus(0, 5);
    run(&mut input.clone(), (0, 0), &mut bus);
    bus.0
}

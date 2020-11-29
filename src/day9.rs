use crate::intcode::*;

#[aoc_generator(day9)]
pub fn gen(input: &str) -> Vec<isize> {
    input.split(',').flat_map(str::parse).collect()
}

#[aoc(day9, part1)]
pub fn part1_impl1(program: &Vec<isize>) -> isize {
    keycode(program.clone(), 1)
}

#[aoc(day9, part2)]
pub fn part2_impl1(program: &Vec<isize>) -> isize {
    keycode(program.clone(), 2)
}

fn keycode(mut program: Vec<isize>, input: isize) -> isize {
    struct BoostIoBus(isize);

    impl IoBus for BoostIoBus {
        fn input(&mut self) -> Option<isize> {
            Some(self.0)
        }

        fn output(&mut self, i: isize) -> bool {
            self.0 = i;
            true
        }
    }

    let mut bus = BoostIoBus(input);
    run(&mut program, (0, 0), &mut bus);
    bus.0
}

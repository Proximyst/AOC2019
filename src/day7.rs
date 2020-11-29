use crate::intcode::*;
use itertools::Itertools as _;

#[aoc_generator(day7)]
pub fn gen(input: &str) -> Vec<isize> {
    input.split(',').flat_map(|s| s.parse()).collect()
}

fn calculate_thrust(
    program: &mut Vec<isize>,
    pc: usize,
    phase: Option<isize>,
    input: isize,
) -> (isize, RunResult) {
    struct PhaseIoBus {
        phase: Option<isize>,
        input_value: isize,
        output: isize,
    };
    impl IoBus for PhaseIoBus {
        fn input(&mut self) -> Option<isize> {
            if let Some(phase) = self.phase {
                self.phase = None;
                return Some(phase);
            }

            Some(self.input_value)
        }

        fn output(&mut self, i: isize) -> bool {
            self.output = i;
            true
        }
    }

    let mut bus = PhaseIoBus {
        phase,
        input_value: input,
        output: 0,
    };
    let result = run(program, (pc, 0), &mut bus);
    (bus.output, result)
}

#[aoc(day7, part1)]
pub fn part1_impl1(program: &Vec<isize>) -> isize {
    let mut max = 0;
    for config in (0..5).permutations(5) {
        let mut result = 0;
        for phase in config {
            result = calculate_thrust(&mut program.clone(), 0, Some(phase), result).0;
        }
        max = std::cmp::max(result, max);
    }

    max
}

#[aoc(day7, part2)]
pub fn part2_impl1(program: &Vec<isize>) -> isize {
    let mut max = 0;
    for config in (5..10).permutations(5) {
        let mut programs = [
            // Sadly Vec is not Copy, so we can't just do [program.clone(); 5]
            program.clone(),
            program.clone(),
            program.clone(),
            program.clone(),
            program.clone(),
        ];
        let mut pc = [0; 5];
        let mut thrust = 0;

        for (idx, &phase) in config.iter().enumerate() {
            let res = calculate_thrust(&mut programs[idx], 0, Some(phase), thrust);
            thrust = res.0;
            pc[idx] = res.1.pc;
        }

        'forever: loop {
            for idx in 0..5 {
                let res = calculate_thrust(&mut programs[idx], pc[idx], None, thrust);
                thrust = res.0;
                pc[idx] = res.1.pc;
                // Not using stopped() is intentional
                if res.1.has_halted {
                    break 'forever;
                }
            }

            max = std::cmp::max(thrust, max);
        }
    }

    max
}

#[test]
fn test_examples() {
    assert_eq!(
        part1_impl1(&vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]),
        43210,
    );
    assert_eq!(
        part1_impl1(&vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]),
        54321,
    );
    assert_eq!(
        part1_impl1(&vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ]),
        65210,
    );

    assert_eq!(
        part2_impl1(&vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ]),
        139629729,
    );
    assert_eq!(
        part2_impl1(&vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ]),
        18216,
    );
}

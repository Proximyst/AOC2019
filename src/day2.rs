use crate::intcode::*;
use rayon::prelude::*;

const GOAL: isize = 19690720;

#[inline(always)]
fn attempt(mut input: Vec<isize>, noun: isize, verb: isize) -> isize {
    input[1] = noun;
    input[2] = verb;
    let _ = run(&mut input, (0, 0), &mut NoIoBusImpl::default());
    input[0]
}

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<isize> {
    input.split(",").flat_map(|s| s.parse::<isize>()).collect()
}

#[aoc(day2, part1)]
pub fn part1_impl1(input: &Vec<isize>) -> isize {
    attempt(input.clone(), 12, 2)
}

#[aoc(day2, part2, rayon)]
pub fn part2_impl1(input: &Vec<isize>) -> isize {
    let (noun, verb, _) = (1isize..=99)
        .into_par_iter()
        .filter_map(|i| {
            (1isize..=99)
                .into_par_iter()
                .map(|j| (j, attempt(input.clone(), i, j)))
                .find_any(|&(_, i)| i == GOAL)
                .map(|(j, res)| (i, j, res))
        })
        .find_any(|&(_, _, res)| res == GOAL)
        .expect("no possible result");

    100 * noun + verb
}

#[aoc(day2, part2, "for loop")]
pub fn part2_impl2(input: &Vec<isize>) -> isize {
    for noun in 1..=99 {
        for verb in 1..=99 {
            if attempt(input.clone(), noun, verb) == GOAL {
                return 100 * noun + verb;
            }
        }
    }

    panic!("No solution possible")
}

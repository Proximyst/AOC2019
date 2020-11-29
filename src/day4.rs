use crate::DigitAtPosition as _;
use rayon::prelude::*;

#[aoc_generator(day4)]
pub fn gen(range: &str) -> (u32, u32) {
    let split = range
        .split('-')
        .flat_map(str::parse)
        .collect::<Vec<_>>();
    assert_eq!(split.len(), 2);

    (split[0], split[1])
}

#[aoc(day4, part1)]
pub fn part1_impl1(&(from, to): &(u32, u32)) -> usize {
    (from..=to).filter(|&i| valid(i, true)).count()
}

#[aoc(day4, part1, rayon)]
pub fn part1_impl2(&(from, to): &(u32, u32)) -> usize {
    (from..=to)
        .into_par_iter()
        .filter(|&i| valid(i, true))
        .count()
}

#[aoc(day4, part2)]
pub fn part2_impl1(&(from, to): &(u32, u32)) -> usize {
    (from..=to).filter(|&i| valid(i, false)).count()
}

#[aoc(day4, part2, rayon)]
pub fn part2_impl2(&(from, to): &(u32, u32)) -> usize {
    (from..=to)
        .into_par_iter()
        .filter(|&i| valid(i, false))
        .count()
}

pub fn valid(num: u32, accept_groups: bool) -> bool {
    // Following requirements are to be imposed:
    //   - There must be six digits (we'll just debug assert this is true)
    //   - Must be within the range given range (we'll just assume this is true
    //     because the numbers are found from the range provided)
    //   - Must have to adjacent digits that are the same
    //   - No digits may ever decrease when read LTR
    //
    // No numbers that are too short should be provided.
    // This is a contract to the function, but we won't exactly care at release
    // given how expensive this check is.
    debug_assert!(num.to_string().len() == 6);

    // For a number: ABCDEF
    // Index 0 will be F, 6 will be A.
    let mut digits = [0u32; 6];
    for (idx, i) in (0..6).map(|i| num.digit_at_pos(i)).enumerate() {
        digits[idx] = i;
    }

    let mut adjacent = false;
    for i in (0..6).rev() {
        let current = digits[i];
        let previous = digits.get(i + 1).copied();
        if let Some(i) = previous {
            if current < i {
                return false;
            }

            // If we accept groups, we'll just check adjacents now. This'll let
            // us avoid heavy and whatnot later.
            if accept_groups && current == i {
                adjacent = true;
            }
        }

        if !accept_groups && !adjacent {
            // We need to fetch all the adjacent digits.
            // We currently have current, previous, and we need the next & its
            // next again (`nnext`).
            let next = digits.get(i.wrapping_sub(1)).copied();
            let nnext = digits.get(i.wrapping_sub(2)).copied();

            // Alright, we have the digits we need.
            //
            // We now have the following cases:
            //   - prev = now & now != next
            //     Impossible, because previous' previous can be this digit,
            //     making a group of 3.
            //   - prev != now & now = next & next != nnext
            //     Possible.
            //   - now != next & next = nnext
            //     Impossible, because the nnext's next may be next's digit,
            //     making a group of 3.

            if previous != Some(current) && Some(current) == next && next != nnext {
                adjacent = true;
            }
        }
    }

    adjacent
}

#[test]
fn valid_numbers() {
    assert!(valid(123356, true));
    assert!(valid(123566, true));
    assert!(!valid(654321, true));
    assert!(!valid(654421, true));

    assert!(valid(123356, false));
    assert!(valid(123366, false));
    assert!(valid(122233, false));
    assert!(!valid(122234, false));
    assert!(!valid(654321, false));
    assert!(!valid(654421, false));
}

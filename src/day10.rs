use noisy_float::prelude::*;
use rayon::prelude::*;
use std::collections::{BTreeMap, HashSet};

#[aoc_generator(day10)]
pub fn gen(input: &str) -> Vec<(f64, f64)> {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#') // c == asteroid
                .map(|(x, _)| (x as f64, y as f64))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

#[aoc(day10, part1)]
pub fn part1_impl1(input: &[(f64, f64)]) -> usize {
    calculate_best_point(input).1
}

#[aoc(day10, part2)]
pub fn part2_impl1(input: &[(f64, f64)]) -> u32 {
    // First we need to find the point in which this is possible.
    // Then let's use (x, y) as our origin when rotating around to find all
    // other points which we can vapourise with our giant laser. To do this,
    // we first need to actually *find* all the points by tracing them.
    //
    // This will not be efficient, but it will be understandable.
    let ((x, y), _) = calculate_best_point(input);

    // We now know our new origin. Time to construct a map of all other
    // asteroids on the plane. We use a BTreeMap for this, as we need keys
    // compared to other keys (i.e. we need to start at π/2 and move downwards
    // until we again hit -π/2). We need to be careful that our origin is not
    // included in this map.
    let mut map = BTreeMap::new();
    for &(ox, oy) in input {
        let (dx, dy) = (ox - x, oy - y);
        if (dx, dy) == (0.0, 0.0) {
            continue;
        }

        // Get the angle between this and the plane; this takes the
        // arctan2 of the delta vector. It's called `phi`, as that's
        // what polar coordinates would call it, but we don't care abouy
        // the radius `r`, so that's never calculated.
        let phi = dy.atan2(dx);
        let vec = map.entry(-n64(phi)).or_insert_with(Vec::new);
        vec.push((ox, oy));
    }

    // We have all the asteroids and their angles from the origin point.
    for vec in map.values_mut() {
        vec.sort_by(|&(ax, ay), &(bx, by)| {
            // We need to sort them by distance. This'll be done through sorting
            // their vector magnitudes from the origin. Because both of them are
            // squared anyways, there's no need to do an expensive sqrt call.
            //
            // Let's first find the deltas of the asteroids.
            let (dax, day) = (ax - x, ay - y);
            let (dbx, dby) = (bx - x, by - y);
            let magnitude_a = n64(dax * dax + day * day);
            let magnitude_b = n64(dbx * dbx + dby * dby);

            magnitude_b.cmp(&magnitude_a)
        });
    }

    // The map is now sorted, so we need to start iteration around in a circle
    // until we hit the 200th asteroid.
    let (x, y) = 'coord: loop {
        let mut count = 0;
        let mut angle = n64(std::f64::consts::FRAC_PI_2);

        loop {
            let range = map.range_mut(..=angle).rev();
            for (_, asteroids) in range {
                if let Some((x, y)) = asteroids.pop() {
                    count += 1;
                    if count == 200 {
                        break 'coord (x, y);
                    }
                }
            }

            // We now need to find the next angle at which to point our laser.
            // Given we know arctan2 returns a value x such that x \in (-π, π],
            // we need to go from -π to +π when we meet the need for it.
            angle = n64(std::f64::consts::PI);
        }
    };
    let (x, y) = (x.trunc() as u32, y.trunc() as u32);

    x * 100 + y
}

/// Calculate the best point to have a monitoring station.
fn calculate_best_point(input: &[(f64, f64)]) -> ((f64, f64), usize) {
    // We need to iterate all the asteroids on our cartesian plane, and use them
    // as a "focus" (origin of the coordinate system) for all the other
    // asteroids, such that we find the asteroid with the highest amount of
    // other asteroids in its direct line of sight.
    //
    // To find whether an asteroid is blocked by another, we "count" (read: we
    // throw away duplicates) the asteroids which have the same angle from the
    // current asteroid compared to the X-axis using arctan2.
    input
        .par_iter() // Use rayon for this; it's slow.
        .map(|&(x, y)| {
            // A HashSet is used for laziness.
            let mut set = HashSet::new();

            // Iterate every other asteroid in the system.
            // We need to be careful to not iterate the asteroid we currently
            // call (0, 0), i.e. the one with a delta of 0.
            for &(ox, oy) in input {
                let (dx, dy) = (ox - x, oy - y);
                if (dx, dy) == (0.0, 0.0) {
                    // Don't check self
                    continue;
                }

                // Get the angle between this and the plane; this takes the
                // arctan2 of the delta vector. It's called `phi`, as that's
                // what polar coordinates would call it, but we don't care abouy
                // the radius `r`, so that's never calculated.
                let phi = dy.atan2(dx);

                // Insert it into the set. Wrap the angle in an `N64` to have a
                // `f64` which can actually be hashed safely, and will panic if
                // it is NaN and debug mode. Who knows what happens on release
                // mode; it's not my problem.
                set.insert(n64(phi));
            }

            // Return both the current asteroid and the amount of asteroids
            // we could see.
            ((x, y), set.len())
        })
        .max_by_key(|&(_, len)| len) // We need to keep the asteroid coordinate
        .unwrap()
}

#[test]
fn test_part1() {
    fn count(input: &str) -> usize {
        part1_impl1(&gen(input))
    }

    assert_eq!(
        count(
            "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."
        ),
        35,
    );
    assert_eq!(
        count(
            ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
        ),
        41,
    );
    assert_eq!(
        count(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
        ),
        210,
    );
    assert_eq!(
        count(
            "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
        ),
        33,
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2_impl1(&gen(".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##")),
        802,
    );
}

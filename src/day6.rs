use indexmap::IndexSet;
use std::collections::HashMap;

// Given:
//   A)B
//   A)C
//   B)D
//   D)F
//
// Make the map:
// {
//   B => A,
//   C => A,
//   D => B,
//   F => D,
// }
// Such that a celestial object may only orbit one other object.
#[aoc_generator(day6)]
pub fn gen(input: &str) -> HashMap<String, String> {
    input
        .lines()
        .map(|s| s.split_at(s.find(')').expect("orbit notation")))
        .fold(HashMap::new(), |mut map, (orbitee, orbiter)| {
            map.insert((&orbiter[1..]).into(), orbitee.into());
            map
        })
}

#[aoc(day6, part1)]
pub fn part1_impl1(map: &HashMap<String, String>) -> u32 {
    let mut count = 0;
    for orbiter in map.keys() {
        count += count_orbits(map, orbiter);
    }

    count
}

#[aoc(day6, part2)]
pub fn part2_impl1(map: &HashMap<String, String>) -> usize {
    // We need to find the "greatest common denominator" of celestial objects,
    // i.e. the closest possible celestial object. The steps it takes to go from
    // "YOU" to "SAN" are the amount of steps it takes from the objects we are
    // orbiting to the GCD.
    //
    // The easiest way to do this would be to create two Vecs which have the
    // steps required until there are no more steps to take for each of us.
    let santa_steps = trace_orbits(map, "SAN");
    let my_steps = trace_orbits(map, "YOU");

    // Now we need to find the first common orbit.
    let mut intersection = santa_steps.intersection(&my_steps);
    let first = intersection.nth(0).expect("a common celestial object");

    santa_steps
        .get_index_of(first)
        .expect("common value to exist in both")
        + my_steps
            .get_index_of(first)
            .expect("common value to exist in both")
}

fn count_orbits(map: &HashMap<String, String>, object: &str) -> u32 {
    let mut count = 0;

    let mut value = map.get(object);
    while let Some(orbiter) = value {
        count += 1;
        value = map.get(orbiter);
    }

    count
}

fn trace_orbits<'a>(map: &'a HashMap<String, String>, object: &str) -> IndexSet<&'a str> {
    let mut set = IndexSet::new();

    let mut value = map.get(object);
    while let Some(orbiter) = value {
        set.insert(orbiter.as_str());
        value = map.get(orbiter);
    }

    set
}

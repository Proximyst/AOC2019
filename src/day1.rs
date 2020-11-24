#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<i64> {
    input
        .lines()
        .flat_map(|s| s.parse::<i64>())
        .map(self::calculate_fuel)
        .collect()
}

#[aoc(day1, part1)]
pub fn part1_impl1(input: &[i64]) -> i64 {
    input.into_iter().sum()
}

#[aoc(day1, part2)]
pub fn part2_impl1(input: &[i64]) -> i64 {
    input
        .into_iter()
        .copied()
        .map(|f| f + calculate_recursive_fuel(f))
        .sum()
}

fn calculate_fuel(i: i64) -> i64 {
    (i / 3) - 2
}

fn calculate_recursive_fuel(i: i64) -> i64 {
    if i <= 0 {
        return 0;
    }

    let f = calculate_fuel(i);
    f + calculate_recursive_fuel(f).max(0)
}

#[test]
fn test() {
    assert_eq!(calculate_fuel(1969), 654);
    assert_eq!(calculate_fuel(12), 2);
    assert_eq!(calculate_fuel(14), 2);
    assert_eq!(calculate_recursive_fuel(12), 2);
    assert_eq!(calculate_recursive_fuel(14), 2);
    assert_eq!(calculate_recursive_fuel(1969), 966);
    assert_eq!(calculate_recursive_fuel(100756), 50346);
}

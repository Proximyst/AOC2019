const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const BLOCK: char = '█';

#[aoc_generator(day8)]
pub fn gen(input: &str) -> Vec<Vec<u8>> {
    let digits: Vec<u8> = input
        .lines()
        .flat_map(str::chars)
        .flat_map(|ch| ch.to_digit(10))
        .map(|i| {
            debug_assert!(i <= u8::MAX as u32);
            i as u8
        })
        .collect();

    let layer_count = digits.len() / (WIDTH * HEIGHT);
    let mut layers = Vec::with_capacity(layer_count);
    for i in 0..layer_count {
        let from = i * WIDTH * HEIGHT;
        let to = from + WIDTH * HEIGHT;
        layers.push((&digits[from..to]).to_vec());
    }

    layers
}

#[aoc(day8, part1)]
pub fn part1_impl1(input: &[Vec<u8>]) -> usize {
    let mut min = (usize::MAX, &[][..]);
    for layer in input {
        let count = count_occurrences(layer, &0);
        if count < min.0 {
            min = (count, layer);
        }
    }

    count_occurrences(min.1, &1) * count_occurrences(min.1, &2)
}

#[aoc(day8, part2)]
pub fn part2_impl1(input: &[Vec<u8>]) -> String {
    let mut display = String::with_capacity(WIDTH * HEIGHT + HEIGHT + 1);
    display.push('\n');
    let count = input.len();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            for i in 0..count {
                let layer = &input[i];
                let pixel = layer[y * WIDTH + x];
                let character = match pixel {
                    0 => ' ',
                    1 => BLOCK,
                    2 => continue, // Transparent
                    n @ _ => panic!("Unknown pixel: {}", n),
                };
                display.push(character);
                break;
            }
        }

        display.push('\n');
    }

    display
}

fn count_occurrences<T>(input: &[T], value: &T) -> usize
where
    T: PartialEq,
{
    input.into_iter().filter(|&i| i == value).count()
}

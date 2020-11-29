use crate::intcode::*;
use std::collections::HashMap;

const BLOCK: char = '█';

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn walk(self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Self::Up => (x, y + 1),
            Self::Left => (x - 1, y),
            Self::Down => (x, y - 1),
            Self::Right => (x + 1, y),
        }
    }
}

pub struct RobotBus {
    grid: HashMap<(isize, isize), bool>,
    coords: (isize, isize),
    direction: Direction,
    walking: bool,
}

impl Default for RobotBus {
    fn default() -> Self {
        Self {
            grid: HashMap::new(),
            coords: (0, 0),
            direction: Direction::Up,
            walking: false,
        }
    }
}

impl IoBus for RobotBus {
    fn input(&mut self) -> Option<isize> {
        Some(self.grid.get(&self.coords).copied().unwrap_or(false) as isize)
    }

    fn output(&mut self, i: isize) -> bool {
        if !self.walking {
            self.grid.insert(self.coords, i == 1);
        } else {
            self.direction = if i == 0 {
                self.direction.left()
            } else {
                self.direction.right()
            };
            self.coords = self.direction.walk(self.coords);
        }
        self.walking = !self.walking;

        false
    }
}

#[aoc_generator(day11)]
pub fn gen(input: &str) -> Vec<isize> {
    input.split(',').flat_map(str::parse).collect()
}

#[aoc(day11, part1)]
pub fn part1_impl1(input: &Vec<isize>) -> usize {
    let mut bus = RobotBus::default();
    run(&mut input.clone(), (0, 0), &mut bus);
    bus.grid.len()
}

#[aoc(day11, part2)]
pub fn part2_impl1(input: &Vec<isize>) -> String {
    let mut bus = RobotBus::default();
    bus.grid.insert((0, 0), true);
    run(&mut input.clone(), (0, 0), &mut bus);

    let min_x = *bus.grid.iter().map(|((x, _), _)| x).min().unwrap();
    let min_y = *bus.grid.iter().map(|((_, y), _)| y).min().unwrap();
    let max_x = *bus.grid.iter().map(|((x, _), _)| x).max().unwrap();
    let max_y = *bus.grid.iter().map(|((_, y), _)| y).max().unwrap();

    let mut display = String::new();
    display.push('\n');
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let white = bus.grid.get(&(x, y)).copied().unwrap_or(false);

            if white {
                display.push(BLOCK);
            } else {
                display.push(' ');
            }
        }
        display.push('\n');
    }

    display
}

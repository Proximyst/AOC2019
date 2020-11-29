#![allow(dead_code)] // I don't really care.

#[macro_use]
extern crate aoc_runner_derive;

pub mod intcode;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;

aoc_lib! {
    year = 2019
}

trait DigitAtPosition {
    fn digit_at_pos(self, pos: u32) -> Self;
}

macro_rules! digit_as_pos_impl {
    ($($types:ty),*) => {
        $(
            impl DigitAtPosition for $types {
                fn digit_at_pos(self, pos: u32) -> Self {
                    #[allow(unused_comparisons)]
                    if self < 0 {
                        panic!("Negative values cannot be checked");
                    }

                    (self / (10 as Self).pow(pos)) % 10
                }
            }
        )*
    };
}

digit_as_pos_impl!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);

#[test]
fn test_digit_at_pos() {
    assert_eq!(3i8.digit_at_pos(0), 3i8);
    assert_eq!(13i8.digit_at_pos(0), 3i8);
    assert_eq!(13i8.digit_at_pos(1), 1i8);
    assert_eq!(13i32.digit_at_pos(7), 0i32);
}

#[test]
#[should_panic(expected = "Negative values cannot be checked")]
fn test_digit_at_pos_panic() {
    let _ = i32::MIN.digit_at_pos(31);
    let _ = i64::MIN.digit_at_pos(31);
    let _ = (-13i8).digit_at_pos(1);
    let _ = (-13i8).digit_at_pos(0);
}

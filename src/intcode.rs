//! # intcode - the esoteric language of Advent of Code 2019
//!
//! Any value in the language may be an instruction. The values are written in
//! a decimal number format of `ABCDE`, where `DE` (`number mod 100`) is the
//! operation. The only operation which is above `10` is `99` (halt), so the
//! language *could* have been implemented with only 4 digits at most, or have
//! another data digit. There are currently 10 operations: See [`self::Instr`].
//!
//! The digits `ABC` represent the parameter modes of the third, second, and
//! first parameters, respectively -- assuming the digit isn't `0` and the
//! parameter is expected.
//!
//! ## Instructions
//!
//! The following instructions exist:
//!
//! | Op code | Name | Parameters | Description |
//! |:-------:|:----:|:-----------|:------------|
//! | `1` | `Add` | `augend` (param), `addend` (param), `destination` (dst) | Adds the augend and addend and places the sum into the destination.
//! | `2` | `Mul` | `multiplicand` (param), `multiplier` (param), `destination` (dst) | Multiplies the multiplicand and the multiplier and places the product into the destination.
//! | `3` | `Input` | `destination` (dst) | Reads input from the I/O bus and places it into the destination.
//! | `4` | `Output` | `input` (param) | Reads the input from the program and sends it to the I/O bus.
//! | `5` | `JNZ` | `cell` (param), `destination` (param) | Reads the cell from the program and jumps to the destination if the cell is not zero.
//! | `6` | `JZ` | `cell` (param), `destination` (param) | Reads the cell from the program and jumps to the destination if the cell is zero.
//! | `7` | `LT` | `lhs` (param), `rhs` (param), `destination` (param) | Compares `lhs < rhs` and writes the result as an integer back to `destination`, where `1` is `true` and `0` is `false`.
//! | `8` | `EQ` | `lhs` (param), `rhs` (param), `destination` (param) | Compares `lhs = rhs` and writes the result as an integer back to `destination`, where `1` is `true` and `0` is `false`.
//! | `9` | `ModRelBas` | `mod` (param) | Modifies the relative base register by the `mod`.
//! | `99` | `Hlt` | | Halts the entire intcode computer.
//!
//! ## Positions
//!
//! A [position](`self::Mod`) (`param`) is provided as either `Immediate`,
//! `Position`, or `Relative`.
//!
//! An `Immediate` position is no position at all, and rather is the raw value
//! it is provided, but it may not be provided as a destination (`dst`).
//!
//! A `Position` position is equivalent to `Immediate` in that the value is read
//! in its raw form, but it points to another cell (whose address is that
//! value). This is the default mode.
//!
//! A `Relative` position however is a pointer to a position `value` cells away
//! from the current relative base register.
//!
//! The position is provided as a digit in the `ABCDE` instruction format.
//! `A` represents the 3rd parameter, `B` the 2nd, and `C` the 1st. The
//! different modes are represented by different values: `0` is `Position`,
//! `1` is `Immediate`, and `2` is `Relative`.

#[derive(Debug, Clone, Copy, Default)]
pub struct RunResult {
    pub pc: usize,
    pub relative_base: usize,
    pub has_halted: bool,
}

#[inline(always)]
pub fn run(
    program: &mut Vec<isize>,
    (pc, relative_base): (usize, usize),
    io_handler: &mut impl IoBus,
) -> RunResult {
    let mut result = RunResult {
        pc,
        relative_base,
        has_halted: false,
    };

    while !result.has_halted {
        let mut inc = true;
        let instr = Instr::parse(&program[result.pc..]);
        match instr {
            Instr::Hlt => {
                result.has_halted = true;
                return result;
            }

            Instr::Add(augend, addend, sum) => {
                let idx = sum.index(result.relative_base);
                program.ensure_min(idx, 0);
                program[idx] = augend.read(&program, result.relative_base)
                    + addend.read(&program, result.relative_base);
            }

            Instr::Mul(multiplicand, multiplier, product) => {
                let idx = product.index(result.relative_base);
                program.ensure_min(idx, 0);
                program[idx] = multiplicand.read(&program, result.relative_base)
                    * multiplier.read(&program, result.relative_base);
            }

            Instr::Input(dst) => match io_handler.input() {
                None => {
                    result.has_halted = true;
                    return result;
                }

                Some(i) => {
                    let idx = dst.index(result.relative_base);
                    program.ensure_min(idx, 0);
                    program[idx] = i;
                }
            },

            Instr::Output(cell) => {
                let value = cell.read(&program, result.relative_base);
                if io_handler.output(value) {
                    result.has_halted = true;
                    return result;
                }
            }

            Instr::JNZ(cell, dst) => {
                let value = cell.read(&program, result.relative_base);
                if value != 0 {
                    inc = false;
                    let pc = dst.read(&program, result.relative_base);
                    debug_assert!(pc >= 0, "pc ({}; {:?}) must be >= 0", pc, dst);
                    result.pc = pc as usize;
                }
            }

            Instr::JZ(cell, dst) => {
                let value = cell.read(&program, result.relative_base);
                if value == 0 {
                    inc = false;
                    let pc = dst.read(&program, result.relative_base);
                    debug_assert!(pc >= 0, "pc ({}; {:?}) must be >= 0", pc, dst);
                    result.pc = pc as usize;
                }
            }

            Instr::LT(lhs, rhs, dst) => {
                let lhs = lhs.read(&program, result.relative_base);
                let rhs = rhs.read(&program, result.relative_base);
                let out = dst.read(&program, result.relative_base);
                debug_assert!(out >= 0, "dst ({}; {:?}) must be >= 0", out, dst);
                program[out as usize] = (lhs < rhs) as isize;
            }

            Instr::EQ(lhs, rhs, dst) => {
                let lhs = lhs.read(&program, result.relative_base);
                let rhs = rhs.read(&program, result.relative_base);
                let out = dst.read(&program, result.relative_base);
                debug_assert!(out >= 0, "dst ({}; {:?}) must be >= 0", out, dst);
                program[out as usize] = (lhs == rhs) as isize;
            }

            Instr::ModRelBas(base) => {
                let new = base.read(&program, result.relative_base);
                debug_assert!(new >= 0, "base ({}; {:?}) must be >= 0", new, base);
                result.relative_base = new as usize;
            }
        }

        if inc {
            result.pc += instr.size();
        }
    }

    result
}

/// This defines an I/O bus for an intcode computer.
pub trait IoBus {
    /// An input instruction has been hit.
    ///
    /// If a `None` value is returned, the computer will halt.
    fn input(&mut self) -> Option<isize>;

    /// An output instruction has been hit.
    ///
    /// If a `true` value is returned, the computer will halt.
    fn output(&mut self, i: isize) -> bool;
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct NoIoBusImpl {
    pub panic: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mod {
    Immediate(isize),
    Position(usize),
    Relative(isize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instr {
    /// Add the two first parameters and put the sum into the destination.
    ///
    /// Op code: `01`.
    Add(Mod, Mod, Mod),

    /// Multiply the two first parameters and put the product into the
    /// destination.
    ///
    /// Op code: `02`.
    Mul(Mod, Mod, Mod),

    /// Read an input from the I/O bus and put it into the destination.
    ///
    /// Op code: `03`.
    Input(Mod),

    /// Output the parameter to the I/O bus.
    ///
    /// Op code: `04`.
    Output(Mod),

    /// Jump to the second parameter if the first parameter is not zero.
    /// Opposite of [`Self::JZ`].
    ///
    /// Op code: `05`.
    JNZ(Mod, Mod),

    /// Jump to the second parameter if the first parameter is zero.
    /// Opposite of [`Self::JNZ`].
    ///
    /// Op code: `06`.
    JZ(Mod, Mod),

    /// Compare the two parameters for less-than and put the result as `u8` into
    /// the destination.
    ///
    /// Op code: `07`.
    LT(Mod, Mod, Mod),

    /// Compare the two parameters for equality and put the result as `u8` into
    /// the destination.
    ///
    /// Op code: `08`.
    EQ(Mod, Mod, Mod),

    /// Adds the parameter to the relative base register.
    ///
    /// Op code: `09`.
    ModRelBas(Mod),

    /// Halt the operation of the program at its current state.
    ///
    /// Op code: `99`.
    Hlt,
}

impl Instr {
    #[inline(always)]
    pub const fn size(self) -> usize {
        match self {
            Self::Add(..) => 1 + 3,
            Self::Mul(..) => 1 + 3,
            Self::Input(..) => 1 + 1,
            Self::Output(..) => 1 + 1,
            Self::JNZ(..) => 1 + 2,
            Self::JZ(..) => 1 + 2,
            Self::LT(..) => 1 + 3,
            Self::EQ(..) => 1 + 3,
            Self::ModRelBas(..) => 1 + 1,
            Self::Hlt => 1,
        }
    }

    /// Parse an instruction.
    ///
    /// The `program` parameter assumes it's a subslice of the entire program,
    /// where the `0`th index is in fact from the program counter of the entire
    /// program.
    #[inline(always)]
    fn parse(program: &[isize]) -> Self {
        debug_assert!(program.len() >= 1);
        let instr = program[0];

        match instr % 100 {
            // Halt is the simplest operation to parse.
            99 => Instr::Hlt,

            1 => {
                let augend = Mod::parse(program, 0);
                let addend = Mod::parse(program, 1);
                let sum = Mod::parse(program, 2);
                Instr::Add(augend, addend, sum)
            }

            2 => {
                let multiplicand = Mod::parse(program, 0);
                let multiplier = Mod::parse(program, 1);
                let product = Mod::parse(program, 2);
                Instr::Mul(multiplicand, multiplier, product)
            }

            3 => Instr::Input(Mod::parse(program, 0)),

            4 => Instr::Output(Mod::parse(program, 0)),

            5 => {
                let cell = Mod::parse(program, 0);
                let destination = Mod::parse(program, 1);
                Instr::JNZ(cell, destination)
            }

            6 => {
                let cell = Mod::parse(program, 0);
                let destination = Mod::parse(program, 1);
                Instr::JZ(cell, destination)
            }

            7 => {
                let lhs = Mod::parse(program, 0);
                let rhs = Mod::parse(program, 1);
                let result = Mod::parse(program, 2);
                Instr::LT(lhs, rhs, result)
            }

            8 => {
                let lhs = Mod::parse(program, 0);
                let rhs = Mod::parse(program, 1);
                let result = Mod::parse(program, 2);
                Instr::EQ(lhs, rhs, result)
            }

            9 => Instr::ModRelBas(Mod::parse(program, 0)),

            op @ _ => panic!("Unsupported op: {} ({}) ({:?})", op, instr, program),
        }
    }
}

impl Mod {
    #[inline(always)]
    fn read(self, memory: &[isize], relbas: usize) -> isize {
        match self {
            Self::Immediate(i) => i,
            _ => memory[self.index(relbas)],
        }
    }

    fn index(self, relbas: usize) -> usize {
        match self {
            Self::Immediate(_) => panic!("Immediate is not an index"),
            Self::Position(idx) => idx,
            Self::Relative(idx) => {
                let idx = idx + relbas as isize;
                debug_assert!(idx >= 0);
                idx as usize
            }
        }
    }

    #[inline(always)]
    fn parse(program: &[isize], param: u32) -> Self {
        // We'll have to cast the param a couple times, but that ensures we can
        // do compile-time bounds checking (i.e. it has to fit in the space of
        // 32-bits).

        // We want at least the instruction (1 value) and the parameter (n
        // values, 0-indexed), so we need to add 2 to accomodate for the instr
        // and the 0-indexing.
        debug_assert!(program.len() >= param as usize + 2);

        let instr = program[0];
        let read = program[param as usize + 1];
        match instr.digit_at_pos(param + 2) {
            0 => {
                debug_assert!(read >= 0, "a position must not be negative");
                Self::Position(read as usize)
            }
            1 => Self::Immediate(read),
            2 => Self::Relative(read),
            n @ _ => panic!("Unknown mode: {}", n),
        }
    }
}

impl IoBus for NoIoBusImpl {
    fn input(&mut self) -> Option<isize> {
        if self.panic {
            panic!("I/O is not accepted")
        } else {
            None
        }
    }

    fn output(&mut self, _: isize) -> bool {
        if self.panic {
            panic!("I/O is not accepted")
        }

        true
    }
}

impl Default for NoIoBusImpl {
    fn default() -> Self {
        NoIoBusImpl { panic: true }
    }
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

trait VecEnsureMin<T> {
    fn ensure_min(&mut self, len: usize, value: T);
}

impl<T> VecEnsureMin<T> for Vec<T>
where
    T: Clone,
{
    fn ensure_min(&mut self, len: usize, value: T) {
        if self.len() - 1 > len {
            return;
        }

        self.resize(len + 1, value);
    }
}

#[test]
fn test_digit_at_pos() {
    assert_eq!(3i8.digit_at_pos(0), 3i8);
    assert_eq!(13i8.digit_at_pos(0), 3i8);
    assert_eq!(13i8.digit_at_pos(0), 3i8);
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

#[test]
#[should_panic]
fn test_no_program() {
    let _ = Instr::parse(&vec![]);
}

#[test]
#[should_panic]
fn test_index_on_immediate() {
    let _ = Mod::Immediate(7).index(0);
}

#[test]
fn test_instructions() {
    assert_eq!(Instr::parse(&[99]), Instr::Hlt);
    assert_eq!(
        Instr::parse(&[1, 1, 1, 1]),
        Instr::Add(Mod::Position(1), Mod::Position(1), Mod::Position(1))
    );
    assert_eq!(
        Instr::parse(&[1001, 1, 1, 1]),
        Instr::Add(Mod::Position(1), Mod::Immediate(1), Mod::Position(1))
    );
    assert_eq!(Instr::parse(&[3, 1]), Instr::Input(Mod::Position(1)));
    assert_eq!(Instr::parse(&[4, 1]), Instr::Output(Mod::Position(1)));
}

#[test]
fn test_day2() {
    let mut code = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let res = run(&mut code, (0, 0), &mut NoIoBusImpl::default());
    assert!(res.has_halted);
    assert_eq!(code[0], 3500);

    let mut code = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let res = run(&mut code, (0, 0), &mut NoIoBusImpl::default());
    assert!(res.has_halted);
    assert_eq!(code[0], 30);

    let real = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 13, 19, 1, 9, 19, 23, 2, 13, 23, 27,
        2, 27, 13, 31, 2, 31, 10, 35, 1, 6, 35, 39, 1, 5, 39, 43, 1, 10, 43, 47, 1, 5, 47, 51, 1,
        13, 51, 55, 2, 55, 9, 59, 1, 6, 59, 63, 1, 13, 63, 67, 1, 6, 67, 71, 1, 71, 10, 75, 2, 13,
        75, 79, 1, 5, 79, 83, 2, 83, 6, 87, 1, 6, 87, 91, 1, 91, 13, 95, 1, 95, 13, 99, 2, 99, 13,
        103, 1, 103, 5, 107, 2, 107, 10, 111, 1, 5, 111, 115, 1, 2, 115, 119, 1, 119, 6, 0, 99, 2,
        0, 14, 0,
    ];
    let mut code = real.clone();
    code[1] = 12;
    code[2] = 2;
    let res = run(&mut code, (0, 0), &mut NoIoBusImpl::default());
    assert!(res.has_halted);
    assert_eq!(code[0], 3790689);
    let mut code = real.clone();
    code[1] = 12;
    code[2] = 2;
    let res = run(&mut code, (0, 0), &mut NoIoBusImpl::default());
    assert!(res.has_halted);
    assert_eq!(code[0], 3790689);
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub type Wire = Vec<Matrix2x2<isize>>;

pub struct WireIntersection {
    wires: (Wire, Wire),
    intersection: (isize, isize),
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Vec<WireIntersection> {
    let mut wires = Vec::new();

    for line in input.lines() {
        let mut coord = (0, 0);
        let mut wire: Wire = Vec::new();
        for paths in line.split(",") {
            let current = coord;
            let length = (&paths[1..]).parse::<isize>().expect("path length");
            match paths.chars().nth(0).expect("path direction") {
                'U' => coord.1 += length,
                'D' => coord.1 -= length,
                'L' => coord.0 += length,
                'R' => coord.0 -= length,
                d @ _ => panic!("Unknown direction: {} ({} of {})", d, paths, line),
            };
            wire.push((current, coord));
        }
        wires.push(wire);
    }

    // I now want to filter out all wires that do not intersect,
    // as those literally do not matter whatsoever.
    //
    // Luckily, I spent seven will-to-live tokens on the `intersection`
    // function, so it's easy to do!
    let mut intersections: Vec<WireIntersection> = Vec::new();
    for wire in &wires {
        // We'll have to iterate every section of this wire and compare to
        // every other wire's sections.
        for &segment in wire {
            for other_wire in &wires {
                if other_wire == wire {
                    // We don't want to compare this wire to itself.
                    // There are no intersections there!
                    continue;
                }

                for &other in other_wire {
                    if let Some(point) = intersection(segment, other) {
                        intersections.push(WireIntersection {
                            wires: (wire.clone(), other_wire.clone()),
                            intersection: point,
                        });
                    }
                }
            }
        }
    }

    intersections
}

#[aoc(day3, part1)]
pub fn part1_impl1(input: &[WireIntersection]) -> isize {
    input
        .iter()
        .map(|intersection| intersection.intersection.0.abs() + intersection.intersection.1.abs())
        .min()
        .expect("no intersections")
}

#[aoc(day3, part2)]
pub fn part2_impl1(input: &[WireIntersection]) -> isize {
    fn count(wire: &Wire, (x, y): (isize, isize)) -> isize {
        // We need to calculate how many steps it takes to go from (0, 0) to
        // intersection with each wire.
        //
        // Because each wire can only change in 1 component at a time, we
        // have a really easy time on figuring it out.
        let mut steps = 0;
        for &(from, to) in wire {
            match (from, to) {
                ((x0, y0), (x1, y1)) if x0 == x1 => {
                    // We know the Y coordinate changed here.
                    let diff = (y1 - y0).abs();
                    steps += diff - (diff - y).abs();
                    if x0 == x {
                        // Only the Y coordinate is different in the
                        // intersection! If the intersection is in the end of
                        // this segment, this will just be the same as
                        // steps += diff.
                        return steps;
                    }
                }

                ((x0, y0), (x1, _)) => {
                    // We know the X coordinate changed here.
                    let diff = (x1 - x0).abs();
                    steps += diff - (diff - x).abs();
                    if y0 == y {
                        // Only the X coordinate is different in the
                        // intersection! If the intersection is in the end of
                        // this segment, this will just be the same as
                        // steps += diff.
                        return steps;
                    }
                }
            }
        }

        steps
    }

    input
        .iter()
        .map(
            |WireIntersection {
                 wires,
                 intersection,
             }| count(&wires.0, *intersection) + count(&wires.1, *intersection),
        )
        .min()
        .expect("no intersections")
}

type Matrix2x2<T> = ((T, T), (T, T));

fn intersection(
    ((x0, y0), (x1, y1)): Matrix2x2<isize>,
    ((x2, y2), (x3, y3)): Matrix2x2<isize>,
) -> Option<(isize, isize)> {
    let pu_x = determinant((
        (((x0, y0), (x1, y1)), ((x0, 1), (x1, 1))),
        (((x2, y2), (x3, y3)), ((x2, 1), (x3, 1))),
    ));
    let pl_x = determinant((
        (((x0, 1), (x1, 1)), ((y0, 1), (y1, 1))),
        (((x2, 1), (x3, 1)), ((y2, 1), (y3, 1))),
    ));
    let pu_y = determinant((
        (((x0, y0), (x1, y1)), ((y0, 1), (y1, 1))),
        (((x2, y2), (x3, y3)), ((y2, 1), (y3, 1))),
    ));
    let pl_y = determinant((
        (((x0, 1), (x1, 1)), ((y0, 1), (y1, 1))),
        (((x2, 1), (x3, 1)), ((y2, 1), (y3, 1))),
    ));
    if pl_x == 0 || pl_y == 0 {
        return None;
    }

    Some((pu_x / pl_x, pu_y / pl_y))
}

fn determinant(matrix: Matrix2x2<Matrix2x2<isize>>) -> isize {
    fn det(((x0, y0), (x1, y1)): Matrix2x2<isize>) -> isize {
        x0 * y1 - x1 * y0
    }

    let m1 = det(matrix.0 .0);
    let m2 = det(matrix.0 .1);
    let m3 = det(matrix.1 .0);
    let m4 = det(matrix.1 .1);

    det(((m1, m2), (m3, m4)))
}

#[test]
fn test_intersection() {
    assert_eq!(
        intersection(((-2, 0), (2, 0)), ((0, 2), (0, -2))),
        Some((0, 0)),
    );
    assert_eq!(
        intersection(((-2, -2), (2, 2)), ((-2, 2), (2, -2))),
        Some((0, 0)),
    );
    assert_eq!(
        intersection(((1, 1), (4, 4)), ((4, 1), (1, 4))),
        Some((2, 2)),
    );

    // Parallell lines do not intersect...
    assert_eq!(intersection(((-2, 2), (2, -2)), ((-3, 3), (3, -3))), None);
    // And because a line^2 is always 0...
    assert_eq!(intersection(((-2, 2), (2, -2)), ((-2, 2), (2, -2))), None);

    // Lines that start and end in a shared point do *not* intersect...
    assert_eq!(
        // We want a "snake" of lines:
        // *------+-----* where + is a head & tail
        intersection(((-2, 0), (0, 0)), ((0, 0), (2, 0))),
        None,
    );
    assert_eq!(intersection(((0, 0), (2, 2)), ((1, 1), (-1, -1))), None);
}

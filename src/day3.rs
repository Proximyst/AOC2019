pub type Matrix2x2<T> = ((T, T), (T, T));

#[derive(Debug, Copy, Clone)]
pub struct WireIntersection {
    point: (f32, f32),
    covered1: f32,
    covered2: f32,
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Vec<WireIntersection> {
    type Wire = Vec<Matrix2x2<f32>>;

    // Fun fact, there will only be two wires!
    // Or so I hope. I expect so. Fuck you if you don't.
    fn wire(line: &str) -> Wire {
        let mut coord = (0.0, 0.0);
        let mut wire: Wire = Vec::new();
        for paths in line.split(",") {
            let current = coord;
            let length: f32 = (&paths[1..]).parse().expect("path length");
            match paths.chars().nth(0).expect("path direction") {
                'U' => coord.1 += length,
                'D' => coord.1 -= length,
                'L' => coord.0 += length,
                'R' => coord.0 -= length,
                d @ _ => panic!("Unknown direction: {} ({} of {})", d, paths, line),
            };
            wire.push((current, coord));
        }

        wire
    }
    let mut lines = input.lines();
    let wire1 = wire(lines.next().expect("wire"));
    let wire2 = wire(lines.next().expect("wire"));

    // I now want to find all the intersections in these two wires.
    //
    // Luckily, I spent seven will-to-live tokens on the `intersection`
    // function, so it's easy to do!
    let mut intersections: Vec<WireIntersection> = Vec::new();
    let mut dist1 = 0.0; // The total distance covered of wire #1.
    for &segment in &wire1 {
        let ((x0, y0), (x1, y1)) = segment;
        let distance = (x1 - x0).abs() + (y1 - y0).abs();

        let mut dist2 = 0.0; // The total distance covered of wire #2.
        for &other in &wire2 {
            // First we need to find the total distance this single wire 2
            // segment may cover, such that we later know what to add to dist2.
            //
            // Remember that this is not the distance till the potential
            // intersection; that may be shorter than this segment!
            let ((x2, y2), (x3, y3)) = other;
            let distance2 = (x3 - x2).abs() + (y3 - y2).abs();

            if let Some(point) = intersection(segment, other) {
                // First we have to determine where the point is on the lines,
                // i.e. how large of a section of the line segments have been
                // covered?
                //
                // First we need to know what has changed; luckily a line
                // segment only moves 1 axis at a time, so one of the two
                // point components is always equal to its equivalents on the
                // line segments.
                let (px, py) = point;
                let delta_p0 = (px - x0).abs() + (py - y0).abs();
                let delta_p1 = (px - x2).abs() + (py - y2).abs();
                let covered1 = delta_p0 + dist1;
                let covered2 = delta_p1 + dist2;

                intersections.push(WireIntersection {
                    point,
                    covered1,
                    covered2,
                });
            }

            dist2 += distance2;
        }

        // At least this distance has now been covered on wire #1, no matter
        // what happens next.
        dist1 += distance;
    }

    intersections
}

#[aoc(day3, part1)]
pub fn part1_impl1(input: &[WireIntersection]) -> f32 {
    input
        .iter()
        .map(|&it| manhattan(((0.0, 0.0), it.point)))
        .min_by(|x, y| x.partial_cmp(y).expect("non-nan value"))
        .expect("no intersections")
}

#[aoc(day3, part2)]
pub fn part2_impl1(input: &[WireIntersection]) -> f32 {
    input
        .iter()
        .map(|&it| it.covered1 + it.covered2)
        .min_by(|x, y| x.partial_cmp(y).expect("non-nan value"))
        .expect("no intersections")
}

fn manhattan(((x0, y0), (x1, y1)): Matrix2x2<f32>) -> f32 {
    (x0 - x1).abs() as f32 + (y0 - y1).abs() as f32
}

fn intersection(
    ((x0, y0), (x1, y1)): Matrix2x2<f32>,
    ((x2, y2), (x3, y3)): Matrix2x2<f32>,
) -> Option<(f32, f32)> {
    let x10 = x1 - x0;
    let y10 = y1 - y0;
    let x32 = x3 - x2;
    let y32 = y3 - y2;

    let denominator = x10 * y32 - x32 * y10;
    if denominator == 0.0 {
        return None; // Collinear
    }
    let dpos = denominator > 0.0;

    let x02 = x0 - x2;
    let y02 = y0 - y2;
    let snum = x10 * y02 - y10 * x02;
    if (snum < 0.0) == dpos {
        return None; // No intersection
    }

    let tnum = x32 * y02 - y32 * x02;
    if (tnum < 0.0) == dpos {
        return None; // No intersection
    }

    if ((snum > denominator) == dpos) || ((tnum > denominator) == dpos) {
        return None; // No intersection
    }

    // There must now be an intersection.
    let t = tnum / denominator;

    Some((x0 + t * x10, y0 + t * y10))
}

#[test]
fn test_intersection() {
    assert_eq!(
        intersection(((-2.0, 0.0), (2.0, 0.0)), ((0.0, 2.0), (0.0, -2.0))),
        Some((0.0, 0.0)),
    );
    assert_eq!(
        intersection(((-2.0, -2.0), (2.0, 2.0)), ((-2.0, 2.0), (2.0, -2.0))),
        Some((0.0, 0.0)),
    );
    assert_eq!(
        intersection(((1.0, 1.0), (4.0, 4.0)), ((4.0, 1.0), (1.0, 4.0))),
        Some((2.5, 2.5)),
    );

    // Parallell lines do not intersect...
    assert_eq!(
        intersection(((-2.0, 2.0), (2.0, -2.0)), ((-3.0, 3.0), (3.0, -3.0))),
        None
    );
    // And because a line^2 is always 0...
    assert_eq!(
        intersection(((-2.0, 2.0), (2.0, -2.0)), ((-2.0, 2.0), (2.0, -2.0))),
        None
    );

    // Lines that start and end in a shared point do *not* intersect...
    assert_eq!(
        // We want a "snake" of lines:
        // *------+-----* where + is a head & tail
        intersection(((-2.0, 0.0), (0.0, 0.0)), ((0.0, 0.0), (2.0, 0.0))),
        None,
    );
    assert_eq!(
        intersection(((0.0, 0.0), (2.0, 2.0)), ((1.0, 1.0), (-1.0, -1.0))),
        None
    );
}

#[test]
fn test_part1() {
    let lines = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let wires = gen(lines);
    assert_eq!(part1_impl1(&wires), 159.0);

    let lines = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    let wires = gen(lines);
    assert_eq!(part1_impl1(&wires), 135.0);
}

#[test]
#[should_panic]
fn test_invalid_wire() {
    let lines = "L15,U15\nR15,D15";
    let wires = gen(lines);
    let _ = part1_impl1(&wires); // Panic!
}

#[test]
fn test_part2() {
    let lines = "R8,U5,L5,D3\nU7,R6,D4,L4";
    let wires = gen(lines);
    assert_eq!(wires[0].covered1 + wires[0].covered2, 30.0);
    assert_eq!(wires[1].covered1 + wires[1].covered2, 40.0);
    assert_eq!(part2_impl1(&wires), 30.0);

    let lines = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let wires = gen(lines);
    assert_eq!(part2_impl1(&wires), 610.0);

    let lines = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    let wires = gen(lines);
    assert_eq!(part2_impl1(&wires), 410.0);
}

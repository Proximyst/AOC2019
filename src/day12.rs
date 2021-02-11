#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CelestialObject {
    pos: (i32, i32, i32),
    vel: (i32, i32, i32),
}

#[aoc_generator(day12)]
pub fn gen(input: &str) -> [CelestialObject; 4] {
    let v: Vec<CelestialObject> = input
        .lines()
        .map(|s| {
            let mut coords: Vec<i32> = s
                .split(',')
                .map(str::trim)
                .flat_map(|s| s.split('=').last())
                .flat_map(|s| s.split('>').rev().last())
                .flat_map(str::parse)
                .rev()
                .collect();
            let x = coords.pop().unwrap();
            let y = coords.pop().unwrap();
            let z = coords.pop().unwrap();

            CelestialObject {
                pos: (x, y, z),
                ..Default::default()
            }
        })
        .collect();
    [v[0], v[1], v[2], v[3]]
}

#[aoc(day12, part1)]
pub fn part1_impl1(input: &[CelestialObject; 4]) -> usize {
    let mut objects = input.clone();
    for _ in 0..1000 {
        apply_gravity(&mut objects);
        objects.iter_mut().for_each(CelestialObject::apply_velocity);
    }

    objects.iter().map(CelestialObject::energy).sum()
}

#[aoc(day12, part2)]
pub fn part2_impl1(input: &[CelestialObject; 4]) -> u64 {
    use std::sync::mpsc::channel;
    use std::thread;

    let (xtx, xrx) = channel();
    let (ytx, yrx) = channel();
    let (ztx, zrx) = channel();

    let xinput = input.clone();
    let yinput = input.clone();
    let zinput = input.clone();

    thread::spawn(move || {
        xtx.send(cycle([
            xinput[0].pos.0,
            xinput[1].pos.0,
            xinput[2].pos.0,
            xinput[3].pos.0,
        ]))
        .unwrap()
    });
    thread::spawn(move || {
        ytx.send(cycle([
            yinput[0].pos.1,
            yinput[1].pos.1,
            yinput[2].pos.1,
            yinput[3].pos.1,
        ]))
        .unwrap()
    });
    thread::spawn(move || {
        ztx.send(cycle([
            zinput[0].pos.2,
            zinput[1].pos.2,
            zinput[2].pos.2,
            zinput[3].pos.2,
        ]))
        .unwrap()
    });

    lcm(
        xrx.recv().unwrap(),
        lcm(yrx.recv().unwrap(), zrx.recv().unwrap()),
    )
}

impl CelestialObject {
    fn apply_velocity(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.pos.2 += self.vel.2;
    }

    fn energy(&self) -> usize {
        (self.pos.0.abs() as usize + self.pos.1.abs() as usize + self.pos.2.abs() as usize)
            * (self.vel.0.abs() as usize + self.vel.1.abs() as usize + self.vel.2.abs() as usize)
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        // (a, b) = (b, a % b)
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn compare<T>(first: &T, second: &T) -> i32
where
    T: Ord,
{
    match second.cmp(first) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn apply_gravity(objects: &mut [CelestialObject; 4]) {
    let vec = objects.clone();
    for CelestialObject { pos, vel } in objects {
        for CelestialObject { pos: other, .. } in &vec {
            if pos == other {
                // We can't apply gravity to ourselves.
                continue;
            }

            vel.0 += compare(&pos.0, &other.0);
            vel.1 += compare(&pos.1, &other.1);
            vel.2 += compare(&pos.2, &other.2);
        }
    }
}

fn cycle(planets: [i32; 4]) -> u64 {
    #[derive(Eq, PartialEq, Copy, Clone)]
    struct Delta(i32, i32);

    fn calculate_gravity(delta: &mut Delta, current: [Delta; 4]) {
        let mut grav = 0;
        for other in &current {
            grav += (other.0 - delta.0).signum();
        }
        delta.1 += grav;
    }

    let planets = [
        Delta(planets[0], 0),
        Delta(planets[1], 0),
        Delta(planets[2], 0),
        Delta(planets[3], 0),
    ];
    let mut current = planets.clone();

    for i in 1.. {
        let clone = current.clone();
        calculate_gravity(&mut current[0], clone);
        let clone = current.clone();
        calculate_gravity(&mut current[1], clone);
        let clone = current.clone();
        calculate_gravity(&mut current[2], clone);
        let clone = current.clone();
        calculate_gravity(&mut current[3], clone);

        if current == planets {
            return i;
        }
    }

    unreachable!()
}

#[test]
fn test_part1() {
    fn new(x: i32, y: i32, z: i32, vx: i32, vy: i32, vz: i32) -> CelestialObject {
        CelestialObject {
            pos: (x, y, z),
            vel: (vx, vy, vz),
        }
    }

    let mut sys = gen("<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>");
    assert_eq!(
        sys,
        [
            new(-1, 0, 2, 0, 0, 0),
            new(2, -10, -7, 0, 0, 0),
            new(4, -8, 8, 0, 0, 0),
            new(3, 5, -1, 0, 0, 0),
        ],
    );
    apply_gravity(&mut sys);
    sys.iter_mut().for_each(CelestialObject::apply_velocity);
    assert_eq!(
        sys,
        [
            new(2, -1, 1, 3, -1, -1),
            new(3, -7, -4, 1, 3, 3),
            new(1, -7, 5, -3, 1, -3),
            new(2, 2, 0, -1, -3, 1),
        ],
    );
}

extern crate elapsed;

use elapsed::measure_time;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn main() {
    println!("AOC 2019 Day 12");

    let test_input = vec![
        Planet {
            pos: Pos { x: -1, y: 0, z: 2 },
            vel: Default::default(),
        },
        Planet {
            pos: Pos {
                x: 2,
                y: -10,
                z: -7,
            },
            vel: Default::default(),
        },
        Planet {
            pos: Pos { x: 4, y: -8, z: 8 },
            vel: Default::default(),
        },
        Planet {
            pos: Pos { x: 3, y: 5, z: -1 },
            vel: Default::default(),
        },
    ];
    let input = vec![
        Planet {
            pos: Pos {
                x: -7,
                y: 17,
                z: -11,
            },
            vel: Default::default(),
        },
        Planet {
            pos: Pos { x: 9, y: 12, z: 5 },
            vel: Default::default(),
        },
        Planet {
            pos: Pos { x: -9, y: 0, z: -4 },
            vel: Default::default(),
        },
        Planet {
            pos: Pos { x: 4, y: 6, z: 0 },
            vel: Default::default(),
        },
    ];

    println!("Test 1: {:?}", part1(test_input.clone(), 10));
    println!("Part 1: {:?}", part1(input.clone(), 1000));

    let (solve_time, output) = measure_time(|| part2(test_input));
    println!("Test 2 answer = {:?}, Solve time = {}", output, solve_time);
    // println!("Part 2: {:?}", part2(input));
}

fn part1(mut input: Vec<Planet>, steps: usize) -> isize {
    for _ in 1..=steps {
        // println!("After {} steps", n);
        for _ in 0..input.len() {
            let mut p1 = input.pop().unwrap();
            input.iter().for_each(|p2| p1.apply_gravity(p2));
            input.push(p1);
            input.rotate_left(1);
        }
        input.iter_mut().for_each(|p| p.apply_velocity());
    }
    input.iter().fold(0, |mut acc, p| {
        acc += p.total_energy();
        acc
    })
}

fn part2(mut input: Vec<Planet>) -> isize {
    let mut set = HashSet::new();
    for n in 0.. {
        for _ in 0..input.len() {
            let mut p1 = input.pop().unwrap();
            input.iter().for_each(|p2| p1.apply_gravity(p2));
            input.push(p1);
            input.rotate_left(1);
        }
        input.iter_mut().for_each(|p| p.apply_velocity());
        if !set.insert(calculate_hash(&input)) {
            return n;
        }
    }
    0
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Debug, PartialEq, Hash, Clone)]
struct Planet {
    pos: Pos,
    vel: Vel,
}

impl Planet {
    fn apply_gravity(&mut self, other: &Planet) {
        match (self.pos.x).cmp(&other.pos.x) {
            Ordering::Equal => (),
            Ordering::Less => self.vel.x += 1,
            Ordering::Greater => self.vel.x -= 1,
        }
        match (self.pos.y).cmp(&other.pos.y) {
            Ordering::Equal => (),
            Ordering::Less => self.vel.y += 1,
            Ordering::Greater => self.vel.y -= 1,
        }
        match (self.pos.z).cmp(&other.pos.z) {
            Ordering::Equal => (),
            Ordering::Less => self.vel.z += 1,
            Ordering::Greater => self.vel.z -= 1,
        }
    }
    fn apply_velocity(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }
    fn total_energy(&self) -> isize {
        (self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs())
            * (self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs())
    }
}

#[derive(Debug, PartialEq, Hash, Clone)]
struct Pos {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Default, PartialEq, Hash, Clone)]
struct Vel {
    x: isize,
    y: isize,
    z: isize,
}

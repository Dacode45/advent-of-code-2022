use advent::args;
use anyhow::Result;
use itertools::Itertools;
use std::collections::{hash_map::RandomState, HashMap, HashSet};
use structopt::StructOpt;

fn main() {
    let opt = args::Opt::from_args();

    let solution = if opt.part2 {
        solve2(&opt)
    } else {
        solve1(&opt)
    };

    println!(
        "Solution [{:?}]: {:?}",
        if opt.part2 { 2 } else { 1 },
        solution
    );
}

#[derive(Default)]
struct Grid {
    head: (i32, i32),
    tail: (i32, i32),
}

impl Grid {
    fn touching(&self) -> bool {
        if self.head == self.tail {
            return true;
        }

        let dist = (self.head.0 - self.tail.0, self.head.1 - self.tail.1);
        let dist = (dist.0.abs(), dist.1.abs());

        match dist {
            (0, 1) => true,
            (1, 0) => true,
            (1, 1) => true,
            _ => false,
        }
    }
}

struct Rope {
    list: Vec<(i32, i32)>,
}

impl Rope {
    fn new() -> Self {
        Self {
            list: vec![(0, 0); 10],
        }
    }

    fn next(&mut self, dir: (i32, i32), pos: &mut Vec<(i32, i32)>) {
        let previous = self.list.clone();

        let mut head = &mut self.list[0];
        head.0 += dir.0;
        head.1 += dir.1;

        for i in 1..10 {
            let last = previous[i - 1];
            let prev = self.list[i - 1];
            let current = &mut self.list[i];
            if !touching(prev, *current) {
                if i == 9 {
                    pos.push(*current);
                }
                *current = last;
            }
            if i == 9 {
                pos.push(*current);
            }
        }
    }
}

fn touching(head: (i32, i32), tail: (i32, i32)) -> bool {
    if head == tail {
        return true;
    }

    let dist = (head.0 - tail.0, head.1 - tail.1);
    let dist = (dist.0.abs(), dist.1.abs());

    match dist {
        (0, 1) => true,
        (1, 0) => true,
        (1, 1) => true,
        _ => false,
    }
}

fn solve1(opt: &advent::args::Opt) -> Result<usize> {
    let content = std::fs::read_to_string(&opt.input)?;
    let mut grid = Grid::default();

    let instructions: Vec<_> = content
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let left = split.next().unwrap();
            let right = split.next().unwrap().parse::<i32>().unwrap();

            match left {
                "L" => (right, (-1, 0)),
                "U" => (right, (0, 1)),
                "R" => (right, (1, 0)),
                "D" => (right, (0, -1)),
                _ => unreachable!("Left: {:?}", left),
            }
        })
        .collect();

    let mut pos = Vec::new();

    for (count, (x, y)) in &instructions {
        for i in 0..*count {
            let current = grid.head.clone();
            grid.head.0 += x;
            grid.head.1 += y;
            if !grid.touching() {
                pos.push(grid.tail.clone());
                grid.tail = current;
            }
            pos.push(grid.tail.clone());
        }
    }

    let visited: HashSet<&(i32, i32)> = std::collections::HashSet::from_iter(pos.iter());

    Ok(visited.len())
}

fn solve2(opt: &advent::args::Opt) -> Result<usize> {
    let content = std::fs::read_to_string(&opt.input)?;
    Ok(drag_rope::<10>(parse(&content)))
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn new(x: isize, y: isize) -> Self {
        Vec2 { x, y }
    }

    fn from_char(c: char) -> Self {
        match c {
            'U' => Self::new(0, 1),
            'R' => Self::new(1, 0),
            'D' => Self::new(0, -1),
            'L' => Self::new(-1, 0),
            _ => panic!("unexpected item in bagging area"),
        }
    }

    fn manhattan_dist(self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

///flattens "R 4" into "R, R, R, R" while parsing
fn parse<'a>(input: &'a str) -> impl Iterator<Item = Vec2> + 'a {
    input.lines().flat_map(|line| {
        std::iter::repeat(Vec2::from_char(line.chars().next().expect("nonempty")))
            .take(line[2..].parse::<usize>().expect("numeric"))
    })
}

fn update_tail(head: Vec2, tail: Vec2) -> Vec2 {
    if (head.x.abs_diff(tail.x) <= 1) && (head.y.abs_diff(tail.y) <= 1) {
        return tail; //no need to move
    }

    //Minimize distance to the head
    let mut steps = vec![
        Vec2::new(0, 1),
        Vec2::new(1, 1),
        Vec2::new(1, 0),
        Vec2::new(1, -1),
        Vec2::new(0, -1),
        Vec2::new(-1, -1),
        Vec2::new(-1, 0),
        Vec2::new(-1, 1),
    ];
    steps.sort_by_key(|step| (tail + *step).manhattan_dist(head));
    tail + steps[0]
}

fn drag_rope<const ROPE_LENGTH: usize>(steps: impl Iterator<Item = Vec2>) -> usize {
    assert!(ROPE_LENGTH >= 2, "nontrivial rope");

    let mut rope = [Vec2::default(); ROPE_LENGTH];
    let mut unique_tail_locations = std::collections::HashSet::<Vec2>::new();
    unique_tail_locations.insert(rope[ROPE_LENGTH - 1]);

    for step in steps {
        rope[0] = rope[0] + step;
        for i in 1..ROPE_LENGTH {
            rope[i] = update_tail(rope[i - 1], rope[i]);
        }

        unique_tail_locations.insert(rope[ROPE_LENGTH - 1]);
    }

    unique_tail_locations.len()
}

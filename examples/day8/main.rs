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

fn solve1(opt: &advent::args::Opt) -> Result<usize> {
    let content = std::fs::read_to_string(&opt.input)?;
    let grid: Vec<Vec<usize>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    let mut count: usize = 0;

    for (r, line) in grid.iter().enumerate() {
        for (c, value) in line.iter().enumerate() {
            if r == 0 || (r == grid.len() - 1) || c == 0 || (c == grid[0].len() - 1) {
                dbg!((r, c));
                count += 1;
                continue;
            }

            let left: Vec<_> = line
                .iter()
                .enumerate()
                .filter(|(x, _)| *x < c)
                .map(|(x, v)| (x, *v))
                .collect();
            let right: Vec<_> = line
                .iter()
                .enumerate()
                .filter(|(x, _)| *x > c)
                .map(|(x, v)| (x, *v))
                .collect();
            let top: Vec<_> = grid
                .iter()
                .map(|line| line[c])
                .enumerate()
                .filter(|(y, _)| *y < r)
                .collect();
            let bottom: Vec<_> = grid
                .iter()
                .map(|line| line[c])
                .enumerate()
                .filter(|(y, _)| *y > r)
                .collect();

            let sides = [left, right, top, bottom];
            if sides.iter().any(|side| side.iter().all(|(_, v)| v < value)) {
                dbg!((r, c));
                count += 1;
            }
        }
    }

    Ok(count)
}

fn solve2(opt: &advent::args::Opt) -> Result<usize> {
    let content = std::fs::read_to_string(&opt.input)?;
    let grid: Vec<Vec<usize>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    let mut best: usize = 0;

    for (r, line) in grid.iter().enumerate() {
        for (c, value) in line.iter().enumerate() {
            if r == 0 || (r == grid.len() - 1) || c == 0 || (c == grid[0].len() - 1) {
                continue;
            }
            let mut score = 1;

            let mut side = 0;
            // left
            for x in (0..c).rev() {
                if grid[r][x] < *value {
                    side += 1;
                } else {
                    side += 1;
                    break;
                }
            }
            dbg!(("left", side));

            score *= side;

            side = 0;
            // right
            for x in ((c + 1)..grid[0].len()) {
                if grid[r][x] < *value {
                    side += 1;
                } else {
                    side += 1;
                    break;
                }
            }
            score *= side;
            dbg!(("right", side));

            let mut side = 0;
            // top
            for y in (0..r).rev() {
                if grid[y][c] < *value {
                    side += 1;
                } else {
                    side += 1;
                    break;
                }
            }
            score *= side;

            dbg!(("top", side));

            side = 0;
            // bottom
            for y in ((r + 1)..grid.len()) {
                if grid[y][c] < *value {
                    side += 1;
                } else {
                    side += 1;
                    break;
                }
            }
            score *= side;

            dbg!(("bottom", side));

            dbg!((r, c, value, score));

            best = best.max(score);
        }
    }

    Ok(best)
}

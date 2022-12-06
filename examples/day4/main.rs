use advent::args;
use anyhow::Result;
use bevy::prelude::system_adapter::new;
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

fn solve1(opt: &advent::args::Opt) -> Result<i32> {
    let content = std::fs::read_to_string(&opt.input)?;

    let value = content
        .lines()
        .map(|line| line.split(","))
        .map(|parts| {
            parts
                .map(|part| {
                    part.split("-")
                        .map(|s| s.parse().expect("NAN"))
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|mut pairs| {
            let first = &pairs[0];
            let second = &pairs[1];
            if (first[0] <= second[0] && first[1] >= second[1])
                || (second[0] <= first[0] && second[1] >= first[1])
            {
                1
            } else {
                0
            }
        })
        .sum();

    Ok(value)
}

fn solve2(opt: &advent::args::Opt) -> Result<i32> {
    let content = std::fs::read_to_string(&opt.input)?;

    let value = content
        .lines()
        .map(|line| line.split(","))
        .map(|parts| {
            parts
                .map(|part| {
                    part.split("-")
                        .map(|s| s.parse().expect("NAN"))
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|pairs| {
            let first = &pairs[0];
            let second = &pairs[1];
            if first[0] <= second[1] && second[0] <= first[1] {
                1
            } else {
                0
            }
        })
        .sum();

    Ok(value)
}

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
    let (idx, _) = content
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .enumerate()
        .find(|(_, s)| HashSet::<&char>::from_iter(s.iter()).len() == 4)
        .expect("Couldn't find any");

    Ok(idx + 4)
}

fn solve2(opt: &advent::args::Opt) -> Result<usize> {
    let content = std::fs::read_to_string(&opt.input)?;
    let (idx, _) = content
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .enumerate()
        .find(|(_, s)| HashSet::<&char>::from_iter(s.iter()).len() == 14)
        .expect("Couldn't find any");

    Ok(idx + 14)
}

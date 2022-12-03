use anyhow::anyhow;
use anyhow::{Ok, Result};

use std::fs;

use bevy::prelude::*;

use advent::args;

pub fn part1(opt: &args::Opt) -> Result<i32> {
    let content = fs::read_to_string(&opt.input)?;
    let elves = content.parse::<Elves>()?;
    elves.0.into_iter().max().ok_or(anyhow!("empty input"))
}

pub fn part2(opt: &args::Opt) -> Result<i32> {
    let content = fs::read_to_string(&opt.input)?;
    let mut elves = content.parse::<Elves>()?;
    elves.0.sort_by(|a, b| b.cmp(a));
    Ok(elves.0.iter().take(3).sum::<i32>())
}

#[derive(Resource)]
pub struct Elves(Vec<i32>);

impl std::str::FromStr for Elves {
    type Err = anyhow::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let calories: Result<Vec<i32>, _> = content
            .split("\n\n")
            .map(|nums| nums.lines().map(str::parse::<i32>).sum())
            .collect();

        Ok(calories.map(Elves)?)
    }
}

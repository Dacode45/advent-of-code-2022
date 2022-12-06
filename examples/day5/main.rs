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

fn solve1(opt: &advent::args::Opt) -> Result<String> {
    let content = std::fs::read_to_string(&opt.input)?;

    let mut lines = content.lines().collect::<Vec<_>>();

    let mut row_strs = lines
        .iter()
        .take_while(|line| !line.contains("1"))
        .collect::<Vec<_>>();

    dbg!(&row_strs);

    let mut row_desc = lines
        .iter()
        .find(|line| line.contains("1"))
        .expect("Cant find descripters");

    let row_desc = row_desc
        .split_whitespace()
        .filter_map(|part| part.parse::<i32>().ok())
        .collect::<Vec<_>>();

    dbg!(&row_desc);

    let mut rows = Vec::with_capacity(row_desc.len());
    for i in 0..row_desc.len() {
        rows.push(Vec::new());
    }

    for row in &row_strs {
        for (idx, letter) in row
            .chars()
            .chunks(4)
            .into_iter()
            .map(|chunk| chunk.skip(1).next().expect("Bad Char"))
            .enumerate()
        {
            if !letter.is_whitespace() {
                dbg!((idx, letter));
                rows[idx].push(letter)
            }
        }
    }

    for mut row in &mut rows {
        row.reverse();
    }

    let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let instructions = lines
        .iter()
        .skip_while(|line| !line.contains("move"))
        .map(|line| {
            let caps = re
                .captures(line)
                .expect(&format!("doesn't match regex: {:?}", line));

            let count = caps
                .get(1)
                .map(|num| num.as_str().parse::<usize>().unwrap())
                .unwrap();
            let from = caps
                .get(2)
                .map(|num| num.as_str().parse::<usize>().unwrap())
                .unwrap();
            let to = caps
                .get(3)
                .map(|num| num.as_str().parse::<usize>().unwrap())
                .unwrap();

            (count, from - 1, to - 1)
        });

    for item in instructions {
        let (count, from, to) = dbg!(item);
        for i in 0..count {
            let thing = rows[from].pop().expect("nothing left");
            rows[to].push(thing);
        }
    }

    let value = rows
        .iter()
        .map(|row| row.last().unwrap())
        .collect::<String>();

    Ok(value)
}

fn solve2(opt: &advent::args::Opt) -> Result<String> {
    let content = std::fs::read_to_string(&opt.input)?;

    let mut lines = content.lines().collect::<Vec<_>>();

    let mut row_strs = lines
        .iter()
        .take_while(|line| !line.contains("1"))
        .collect::<Vec<_>>();

    dbg!(&row_strs);

    let mut row_desc = lines
        .iter()
        .find(|line| line.contains("1"))
        .expect("Cant find descripters");

    let row_desc = row_desc
        .split_whitespace()
        .filter_map(|part| part.parse::<i32>().ok())
        .collect::<Vec<_>>();

    dbg!(&row_desc);

    let mut rows = Vec::with_capacity(row_desc.len());
    for i in 0..row_desc.len() {
        rows.push(Vec::new());
    }

    for row in &row_strs {
        for (idx, letter) in row
            .chars()
            .chunks(4)
            .into_iter()
            .map(|chunk| chunk.skip(1).next().expect("Bad Char"))
            .enumerate()
        {
            if !letter.is_whitespace() {
                dbg!((idx, letter));
                rows[idx].push(letter)
            }
        }
    }

    for mut row in &mut rows {
        row.reverse();
    }

    let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let instructions = lines
        .iter()
        .skip_while(|line| !line.contains("move"))
        .map(|line| {
            let caps = re
                .captures(line)
                .expect(&format!("doesn't match regex: {:?}", line));

            let count = caps
                .get(1)
                .map(|num| num.as_str().parse::<usize>().unwrap())
                .unwrap();
            let from = caps
                .get(2)
                .map(|num| num.as_str().parse::<usize>().unwrap())
                .unwrap();
            let to = caps
                .get(3)
                .map(|num| num.as_str().parse::<usize>().unwrap())
                .unwrap();

            (count, from - 1, to - 1)
        });

    for item in instructions {
        let (count, from, to) = dbg!(item);
        let idx = rows[from].len() - count;
        let row = rows[from].clone();
        let parts = dbg!(row.split_at(idx));

        rows[from] = Vec::from(parts.0);
        rows[to].extend(parts.1);
    }

    let value = rows
        .iter()
        .map(|row| row.last().unwrap())
        .collect::<String>();

    Ok(value)
}

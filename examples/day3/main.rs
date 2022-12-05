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

fn solve1(opt: &advent::args::Opt) -> Result<u32> {
    let content = std::fs::read_to_string(&opt.input)?;

    let value = content
        .lines()
        .map(|line| dbg!(line).split_at(line.len() / 2))
        .map(|(a, b)| {
            a.chars()
                .find(|a_char| b.find(|b_char| *a_char == b_char).is_some())
                .expect(&format!("No similarities in compartment: {:?}/{:?}", a, b))
        })
        .map(|char| {
            if char.is_ascii_uppercase() {
                dbg!(char);
                dbg!((char as u32) - ('A' as u32) + 27)
            } else {
                dbg!(char);
                dbg!((char as u32) - ('a' as u32) + 1)
            }
        })
        .sum();

    Ok(value)
}

fn solve2(opt: &advent::args::Opt) -> Result<u32> {
    let content = std::fs::read_to_string(&opt.input)?;

    let mut map = HashMap::new();

    let value = &content
        .lines()
        .map(|line| HashSet::<char, RandomState>::from_iter(line.chars()))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|sets| {
            dbg!(&sets);
            map.clear();
            sets.iter().for_each(|set| {
                set.iter().for_each(|char| {
                    map.entry(char).and_modify(|v| *v += 1).or_insert(1);
                })
            });
            map.iter()
                .find(|(k, v)| **v == 3)
                .map(|(k, v)| k.clone())
                .expect(&format!("No char occured 3 times in {:?}", &sets))
        })
        .map(|char| {
            if char.is_ascii_uppercase() {
                dbg!(char);
                dbg!((*char as u32) - ('A' as u32) + 27)
            } else {
                dbg!(char);
                dbg!((*char as u32) - ('a' as u32) + 1)
            }
        })
        .sum();

    Ok(*value)
}

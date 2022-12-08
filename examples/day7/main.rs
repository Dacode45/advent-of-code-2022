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

enum DirEnt {
    Dir(Vec<String>),
    File(usize, String),
}

fn solve1(opt: &advent::args::Opt) -> Result<usize> {
    use DirEnt::*;
    let cd_re = regex::Regex::new(r"\$ cd ([/|..|\w]+)").unwrap();
    let ls_re = regex::Regex::new(r"\$ ls").unwrap();
    let dir_re = regex::Regex::new(r"dir (\w+)").unwrap();
    let file_re = regex::Regex::new(r"(\d+) (\w+(\.\w+)?)").unwrap();

    let mut system = HashMap::<String, DirEnt>::new();
    system.insert("".to_owned(), Dir(Vec::new()));

    let mut current = vec![""];

    let content = std::fs::read_to_string(&opt.input)?;
    for line in content.lines() {
        dbg!(&line);
        if let Some(captures) = cd_re.captures(line) {
            let goto = captures.get(1).unwrap();
            match goto.as_str() {
                "/" => current = vec![""],
                ".." => {
                    current.pop();
                }
                next => {
                    current.push(next);
                    println!("CD dir {:?}", &current.join("/"));

                    let dir = system.get(&current.join("/"));
                    assert!(dir.is_some());
                    assert!(matches!(dir, Some(Dir(..))));
                }
            }
        } else if let Some(captures) = ls_re.captures(line) {
        } else if let Some(captures) = dir_re.captures(line) {
            let part = captures.get(1).unwrap();
            let next = current.join("/") + "/" + part.as_str();
            println!("Found dir {:?}", next);
            system.entry(next).or_insert(Dir(Vec::new()));
        } else if let Some(captures) = file_re.captures(line) {
            let num = captures.get(1).unwrap().as_str().parse().unwrap();
            let name = captures.get(2).unwrap();
            let next = current.join("/") + "/" + name.as_str();
            assert!(!system.contains_key(&next));
            system.insert(next, File(num, name.as_str().to_owned()));
        } else {
            unreachable!("No regex matches");
        }
    }

    let mut sizes = HashMap::new();

    for (name, entry) in &system {
        if let File(size, last) = entry {
            let parts: Vec<_> = name.split("/").collect();
            for i in 0..parts.len() {
                let current = parts[..i].join("/");
                sizes
                    .entry(current)
                    .and_modify(|e| *e += *size)
                    .or_insert(*size);
            }
        }
    }

    let value = sizes
        .iter()
        .filter(|(k, v)| **v <= 100_000)
        .map(|(k, v)| {
            dbg!(k);
            v
        })
        .sum();

    Ok(value)
}

fn solve2(opt: &advent::args::Opt) -> Result<usize> {
    use DirEnt::*;
    let cd_re = regex::Regex::new(r"\$ cd ([/|..|\w]+)").unwrap();
    let ls_re = regex::Regex::new(r"\$ ls").unwrap();
    let dir_re = regex::Regex::new(r"dir (\w+)").unwrap();
    let file_re = regex::Regex::new(r"(\d+) (\w+(\.\w+)?)").unwrap();

    let mut system = HashMap::<String, DirEnt>::new();
    system.insert("".to_owned(), Dir(Vec::new()));

    let mut current = vec![""];

    let content = std::fs::read_to_string(&opt.input)?;
    for line in content.lines() {
        dbg!(&line);
        if let Some(captures) = cd_re.captures(line) {
            let goto = captures.get(1).unwrap();
            match goto.as_str() {
                "/" => current = vec![""],
                ".." => {
                    current.pop();
                }
                next => {
                    current.push(next);
                    println!("CD dir {:?}", &current.join("/"));

                    let dir = system.get(&current.join("/"));
                    assert!(dir.is_some());
                    assert!(matches!(dir, Some(Dir(..))));
                }
            }
        } else if let Some(captures) = ls_re.captures(line) {
        } else if let Some(captures) = dir_re.captures(line) {
            let part = captures.get(1).unwrap();
            let next = current.join("/") + "/" + part.as_str();
            println!("Found dir {:?}", next);
            system.entry(next).or_insert(Dir(Vec::new()));
        } else if let Some(captures) = file_re.captures(line) {
            let num = captures.get(1).unwrap().as_str().parse().unwrap();
            let name = captures.get(2).unwrap();
            let next = current.join("/") + "/" + name.as_str();
            assert!(!system.contains_key(&next));
            system.insert(next, File(num, name.as_str().to_owned()));
        } else {
            unreachable!("No regex matches");
        }
    }

    let mut sizes = HashMap::new();

    for (name, entry) in &system {
        if let File(size, last) = entry {
            let parts: Vec<_> = name.split("/").collect();
            for i in 0..parts.len() {
                let current = parts[..i].join("/");
                dbg!(&current, size);
                sizes
                    .entry(current)
                    .and_modify(|e| *e += *size)
                    .or_insert(*size);
            }
        }
    }

    let root_size = dbg!(sizes[""] / 2);
    dbg!(70000000 - root_size);
    let value = sizes
        .iter()
        .map(|d| dbg!(d))
        .filter(|(k, v)| (70000000 - root_size) + **v >= 30000000)
        .map(|(k, v)| v)
        .min()
        .unwrap();

    Ok(*value)
}

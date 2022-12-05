use advent::args;

use anyhow::Result;
use bevy::{
    prelude::*,
};
use itertools::Itertools;

use std::collections::HashMap;
use structopt::StructOpt;

fn main() {
    let opt = args::Opt::from_args();

    if opt.compute {
        app(&opt);
        return;
    }

    let solution = solve(&opt).unwrap();

    println!(
        "Solution [{:?}]: {:?}",
        if opt.part2 { 2 } else { 1 },
        solution
    );
}

fn solve(opt: &advent::args::Opt) -> Result<(std::collections::HashMap<&'static str, RPS>, i32)> {
    let content = std::fs::read_to_string(&opt.input)?;
    let rounds = content.parse::<Rounds>()?;

    if opt.part2 {
        Ok(rounds.solve2())
    } else {
        Ok(rounds.solve1())
    }
}

fn app(_Opt: &advent::args::Opt) -> Result<()> {
    todo!("part 2")
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use RPS::*;
        match (self, other) {
            (Rock, Scissors) => Some(std::cmp::Ordering::Greater),
            (Rock, Paper) => Some(std::cmp::Ordering::Less),
            (Scissors, Rock) => Some(std::cmp::Ordering::Less),
            (Scissors, Paper) => Some(std::cmp::Ordering::Greater),
            (Paper, Scissors) => Some(std::cmp::Ordering::Less),
            (Paper, Rock) => Some(std::cmp::Ordering::Greater),
            _ => Some(std::cmp::Ordering::Equal),
        }
    }
}

impl RPS {
    fn value(self) -> i32 {
        use RPS::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn lose(self) -> RPS {
        use RPS::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn win(self) -> RPS {
        use RPS::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn draw(self) -> RPS {
        self
    }

    fn result(self, other: RPS) -> i32 {
        

        match (self, other) {
            (x, y) if x == y => 3 + self.value(),
            (x, y) if x > y => 6 + self.value(),
            (x, y) if x < y => self.value(),
            _ => unreachable!("No result for {:?} / {:?}", self, other),
        }
    }
}

struct Rounds(Vec<(RPS, String)>);

impl std::str::FromStr for Rounds {
    type Err = anyhow::Error;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        use RPS::*;
        let rounds = content
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .map(|line| {
                let first = match line[0] {
                    "A" => Rock,
                    "B" => Paper,
                    "C" => Scissors,
                    _ => unreachable!("Bad input {:?}", line),
                };
                (first, line[1].to_owned())
            })
            .collect();

        Ok(Rounds(rounds))
    }
}

impl Rounds {
    fn score(&self, mapping: &HashMap<&'static str, RPS>) -> i32 {
        self.0.iter().fold(0, |acc, (theirs, mine)| {
            acc + mapping[mine.as_str()].result(*theirs)
        })
    }

    fn solve1(&self) -> (HashMap<&'static str, RPS>, i32) {
        use RPS::*;
        let mapping = HashMap::from([("X", Rock), ("Y", Paper), ("Z", Scissors)]);
        let score = self.score(&mapping);
        (mapping, score)
    }

    fn solve2(&self) -> (HashMap<&'static str, RPS>, i32) {
        use RPS::*;
        let mut mapping = HashMap::from([("X", Rock), ("Y", Paper), ("Z", Scissors)]);

        let mut score = 0;
        for round in &self.0 {
            let mine = if round.1 == "X" {
                round.0.lose()
            } else if round.1 == "Y" {
                round.0.draw()
            } else if round.1 == "Z" {
                round.0.win()
            } else {
                unreachable!("Can have this")
            };
            score += mine.result(round.0);
            *mapping.get_mut(round.1.as_str()).unwrap() = mine;
        }

        (mapping, score)
    }

    fn being_extra(&self) -> (HashMap<String, RPS>, i32) {
        use RPS::*;
        let candidates = [Rock, Paper, Scissors];
        let permutations = (0..3).permutations(3);

        // let mut mapping = HashMap::<&str, _>::new();
        let scores = permutations
            .map(|perm| {
                let mapping = HashMap::from([
                    ("X", candidates[perm[0]]),
                    ("Y", candidates[perm[1]]),
                    ("Z", candidates[perm[2]]),
                ]);

                (perm, self.score(&mapping))
            })
            .collect::<HashMap<_, _>>();

        dbg!(&scores
            .iter()
            .map(|(x, y)| (x.iter().map(|z| candidates[*z]).collect::<Vec<_>>(), y))
            .collect::<HashMap<_, _>>());

        let best = scores
            .iter()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .expect("No max?");

        (
            HashMap::from([
                ("X".to_owned(), candidates[best.0[0]]),
                ("Y".to_owned(), candidates[best.0[1]]),
                ("Z".to_owned(), candidates[best.0[2]]),
            ]),
            *best.1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_paper_scissors() {
        use RPS::*;
        assert!(Rock > Scissors);
        assert!(Scissors > Paper);
        assert!(Paper > Rock);
        assert!(Scissors < Rock);
        assert!(Paper < Scissors);
        assert!(Rock < Paper);
        assert!(Rock == Rock);
        assert!(Scissors == Scissors);
        assert!(Paper == Paper);
    }

    #[test]
    fn scores() {
        use RPS::*;
        assert_eq!(Paper.result(Rock), 8);
        assert_eq!(Rock.result(Paper), 1);
        assert_eq!(Scissors.result(Scissors), 6);
    }
}

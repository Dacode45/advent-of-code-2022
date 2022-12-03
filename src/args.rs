use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "advent", about = "Advent of Code 2022.")]
pub struct Opt {
    #[structopt(long)]
    pub part2: bool,

    #[structopt(long)]
    pub compute: bool,
    /// Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
}

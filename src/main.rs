use std::{collections::HashSet, path::PathBuf};

use oeis_utils::NumberValue;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}
fn main() {
    let opt = Opt::from_args();
    let oeis_db = oeis_utils::OEISDatabase::from_path(&opt.input).unwrap();
    let ss = oeis_db.series;
    let all_nums: HashSet<i128> = ss
        .iter()
        .flat_map(|s| s.values())
        .filter_map(|e| match e {
            NumberValue::InRange(i) => Some(i),
            NumberValue::OutOfRange(_) => None, // If we don't fit in a i128 we are not going to be the smallest number
        })
        .collect();
    let res = (0_i128..).find(|e| !all_nums.contains(e)).unwrap();

    println!("{} not found!", res);
}

use std::collections::HashSet;

use num_bigint::{BigInt, Sign, ToBigInt};
use oeis_utils::{NumberValue};

fn main() {
    let oeis_db = oeis_utils::OEISDatabase::from_path("./stripped").unwrap();
    let ss = oeis_db.series();
    let all_nums: HashSet<i128> = ss
        .iter()
        .flat_map(|s| s.values())
        .filter_map(|e| match e {
            NumberValue::InRange(i) => Some(i),
            NumberValue::OutOfRange(_) => None,
        })
        .collect();
    let res = (0_i128..)
        .find(|e| !all_nums.contains(e))
        .unwrap();

    println!("{} not found!", res);
}

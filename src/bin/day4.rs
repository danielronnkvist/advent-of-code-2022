use std::str::FromStr;
use std::ops::RangeInclusive as Range;

fn fully_contains(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.contains(&b.start()) && a.contains(&b.end())
}

fn has_overlaps(a: &Range<u32>, b: &Range<u32>) -> bool {
    for x in a.clone() {
        if b.contains(&x) {
            return true;
        }
    }
    false
}

fn main() {
    let input = include_str!("./../../inputs/day4.txt").trim();
    let mut result: u32 = 0;
    let mut result2: u32 = 0;
    for line in input.split("\n") {
        let ids: Vec<Range<u32>> = line.split(",").map(|id| {
            let ids: Vec<u32> = id.split("-").map(u32::from_str).map(Result::unwrap).collect();
            ids[0]..=ids[1]
        }).collect();
        if fully_contains(&ids[0], &ids[1]) || fully_contains(&ids[1], &ids[0]) {
            result += 1;
        }
        if has_overlaps(&ids[0], &ids[1]) {
            result2 += 1;
        }

    }
    dbg!(result);
    dbg!(result2);
}

use std::collections::HashSet;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let mut dir = HashMap::new();
    dir.insert("R", (1, 0));
    dir.insert("L", (-1, 0));
    dir.insert("D", (0, -1));
    dir.insert("U", (0, 1));
    let input = include_str!("./../../inputs/day9.txt").trim();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let mut head: (i32, i32) = (0, 0);
    let mut tail = (0, 0);
    for line in input.split("\n") {
        let (d, a) = line.split_at(1);
        let a = u32::from_str(a.trim()).unwrap();
        let d = dir.get(d).unwrap();

        for _ in 0..a {
            let prev = head.clone();
            head.0 += d.0;
            head.1 += d.1;

            let xdiff = head.0.abs_diff(tail.0);
            let ydiff = head.1.abs_diff(tail.1);
            if xdiff > 1 || ydiff > 1 {
                tail = prev;
            }
            visited.insert(tail);
        }
    }
    dbg!(visited.len());
}

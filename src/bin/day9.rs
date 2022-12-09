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
    let mut p1: HashSet<(i32, i32)> = HashSet::new();
    p1.insert((0, 0));
    let mut p2: HashSet<(i32, i32)> = HashSet::new();
    p2.insert((0, 0));
    let mut head: (i32, i32) = (0, 0);
    let mut tail = vec![(0, 0); 9];
    for line in input.split("\n") {
        let (d, a) = line.split_at(1);
        let a = u32::from_str(a.trim()).unwrap();
        let d = dir.get(d).unwrap();

        for _ in 0..a {
            head.0 += d.0;
            head.1 += d.1;

            for i in 0..tail.len() {
                let mut h = head.clone();
                if i > 0 {
                    h = tail[i - 1];
                }
                let t = tail.get_mut(i).unwrap();
                let xdiff = h.0.abs_diff(t.0);
                let ydiff = h.1.abs_diff(t.1);
                if xdiff > 1 || ydiff > 1 {
                    t.0 += (h.0 - t.0) / std::cmp::max((h.0 - t.0).abs(), 1);
                    t.1 += (h.1 - t.1) / std::cmp::max((h.1 - t.1).abs(), 1);
                }
                if i == 0 {
                    p1.insert(*t);
                }
                if i == 8 {
                    p2.insert(*t);
                }
            }
        }
    }
    dbg!(p1.len());
    dbg!(p2.len());
}


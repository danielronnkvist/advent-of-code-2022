use std::str::FromStr;

fn main() {
    let input = include_str!("./../../inputs/day1.txt");
    let mut elfs = vec![0];
    for line in input.split("\n") {
        if line.is_empty() {
            elfs.push(0);
        } else if let Some(elf) = elfs.last_mut() {
            *elf += u32::from_str(line).unwrap();
        }
    }
    elfs.sort();
    elfs.reverse();
    dbg!(elfs[0]);
    dbg!(elfs[0] + elfs[1] + elfs[2]);
}

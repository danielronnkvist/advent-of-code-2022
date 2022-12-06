use std::collections::VecDeque;

fn find_seq(message: &str, length: usize) -> usize {
    let mut markers = VecDeque::new();
    for (i, c) in message.chars().enumerate() {
        if markers.contains(&c) {
            'w: while let Some(c0) = markers.pop_front() {
                if c0 == c {
                    break 'w;
                }
            }
        }
        markers.push_back(c);
        if markers.len() == length {
            return i + 1;
        }
    }
    panic!("nothing found");
}

fn main() {
    let input = include_str!("./../../inputs/day6.txt").trim();
    dbg!(find_seq(input, 4));
    dbg!(find_seq(input, 14));
}

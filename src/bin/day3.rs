fn get_priority(c: char) -> u32 {
    let priority: u32 = c.into();
    if c.is_uppercase() {
        priority - 38
    } else {
        priority - 96
    }
}

fn main() {
    let input = include_str!("./../../inputs/day3.txt");
    let mut result: u32 = 0;
    let mut previous_elf_group: Vec<&str> = vec![];
    let mut result2: u32 = 0;
    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }
        let (a, b) = line.split_at(line.len() / 2);
        'chars: for c in a.chars() {
            if b.contains(c) {
                let priority = get_priority(c);
                result += priority;
                break 'chars;
            }
        }

        if previous_elf_group.len() < 3 {
            previous_elf_group.push(line);
        }
        if previous_elf_group.len() == 3 {
            'chars: for c in previous_elf_group[0].chars() {
                if previous_elf_group[1].contains(c) && previous_elf_group[2].contains(c) {
                    result2 += get_priority(c);
                    break 'chars;
                }
            }

            previous_elf_group = vec![];
        }
    }
    dbg!(result);
    dbg!(result2);
}

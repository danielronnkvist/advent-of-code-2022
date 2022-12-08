fn main() {
    let input = include_str!("./../../inputs/day8.txt").trim();
    let mut trees: Vec<Vec<u32>> = vec![];
    for line in input.split("\n") {
        trees.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut result = 0;
    for (y, row) in trees.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let mut visible = !row[0..x].iter().any(|t| t >= tree);
            if !visible {
                visible = !row[(x + 1)..].iter().any(|t| t >= tree);
            }
            if !visible {
                visible = !trees[0..y].iter().map(|r| r[x]).any(|t| t >= *tree);
            }
            if !visible {
                visible = !trees[(y + 1)..].iter().map(|r| r[x]).any(|t| t >= *tree);
            }
            if visible {
                result += 1;
            }
        }
    }

    dbg!(result);
}

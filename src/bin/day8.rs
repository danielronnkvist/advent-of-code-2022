fn distance(x: usize, y: usize, trees: &Vec<Vec<u32>>) -> u32 {
    let mut dl = x;
    for ix in (0..x).rev() {
        if trees[y][ix] >= trees[y][x] {
            dl = x - ix;
            break;
        }
    }
    let mut dr = trees[y].len() - x - 1;
    for ix in (x+1)..trees[y].len() {
        if trees[y][ix] >= trees[y][x] {
            dr = ix - x;
            break;
        }
    }
    let mut dt = y;
    for iy in (0..y).rev() {
        if trees[iy][x] >= trees[y][x] {
            dt = y - iy;
            break;
        }
    }
    let mut db = trees.len() - y - 1;
    for iy in (y+1)..trees.len() {
        if trees[iy][x] >= trees[y][x] {
            db = iy - y;
            break;
        }
    }

    (dl * dr * dt * db) as u32
}

fn main() {
    let input = include_str!("./../../inputs/day8.txt").trim();
    let mut trees: Vec<Vec<u32>> = vec![];
    for line in input.split("\n") {
        trees.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut result = 0;
    let mut result2 = 0;
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
                let d = distance(x, y, &trees);
                if d > result2 {
                    result2 = d;
                }
            }
        }
    }

    dbg!(result);
    dbg!(result2);
}

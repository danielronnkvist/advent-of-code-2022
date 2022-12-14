use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Material {
    Sand,
    Rock,
    Air,
}

fn main() {
    let input = include_str!("./../../inputs/day14.txt").trim();
    let mut grid = vec![vec![Material::Air; 750]; 750];
    for line in input.split("\n") {
        let rock_paths: Vec<Vec<usize>> = line
            .split(" -> ")
            .map(|coords| {
                coords
                    .split(",")
                    .map(|c| usize::from_str(c).unwrap())
                    .collect()
            })
            .collect();
        for i in 1..rock_paths.len() {
            let from = rock_paths[i - 1].clone();
            let to = rock_paths[i].clone();
            let ymin = {
                if from[1] < to[1] {
                    from[1]
                } else {
                    to[1]
                }
            };
            let ymax = {
                if from[1] > to[1] {
                    from[1]
                } else {
                    to[1]
                }
            };
            let xmin = {
                if from[0] < to[0] {
                    from[0]
                } else {
                    to[0]
                }
            };
            let xmax = {
                if from[0] > to[0] {
                    from[0]
                } else {
                    to[0]
                }
            };

            for y in ymin..=ymax {
                for x in xmin..=xmax {
                    grid[y][x] = Material::Rock;
                }
            }
        }
    }

    let low_point = 750
        - grid
            .iter()
            .rev()
            .position(|m| m.iter().find(|&m| m == &Material::Rock).is_some())
            .unwrap();
    let floor = low_point + 1;
    let start: (usize, usize) = (500, 0);

    let mut p1_grid = grid.clone();
    let mut p1 = 0;
    'o: loop {
        let mut sand = start.clone();

        'i: loop {
            if p1_grid[sand.1][sand.0] != Material::Air {
                if p1_grid[sand.1][sand.0 - 1] == Material::Air {
                    sand.0 -= 1;
                } else if p1_grid[sand.1][sand.0 + 1] == Material::Air {
                    sand.0 += 1;
                } else {
                    p1_grid[sand.1 - 1][sand.0] = Material::Sand;
                    break 'i;
                }
            }
            if sand.1 > low_point {
                break 'o;
            }
            sand.1 += 1;
        }
        p1 += 1;
    }

    let mut p2_grid = grid.clone();
    let mut p2 = 0;
    'o: loop {
        let mut sand = start.clone();

        'i: loop {
            if p2_grid[sand.1][sand.0] != Material::Air || sand.1 == floor {
                if p2_grid[sand.1][sand.0 - 1] == Material::Air && sand.1 != floor {
                    sand.0 -= 1;
                } else if p2_grid[sand.1][sand.0 + 1] == Material::Air && sand.1 != floor {
                    sand.0 += 1;
                } else {
                    p2_grid[sand.1 - 1][sand.0] = Material::Sand;
                    if sand.1 - 1 == start.1 && sand.0 == start.0 {
                        p2 += 1;
                        break 'o;
                    }
                    break 'i;
                }
            }
            sand.1 += 1;
        }
        p2 += 1;
    }

    dbg!(p1);
    assert_eq!(p1, 838);
    dbg!(p2);
    assert_eq!(p2, 27539);
}

use std::collections::{ HashMap, BinaryHeap };

fn get_edges(point: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .filter_map(|dir| {
            let x = point.0 as i32 + dir.0;
            if x >= 0 && x < grid[0].len() as i32 {
                let y = point.1 as i32 + dir.1;
                if y >= 0 && y < grid.len() as i32 {
                    return Some((x as usize, y as usize));
                }
            }
            None
        })
        .collect()
}

fn solve_for(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut visited = vec![];
    let mut to_visit = BinaryHeap::new();
    let mut distances = HashMap::new();

    to_visit.push(Visit {
        vertex: start,
        distance: 0,
    });
    distances.insert(start, 0);

    while let Some(Visit { vertex, distance }) = to_visit.pop() {
        if visited.contains(&vertex) {
            continue;
        } else {
            visited.push(vertex);
        }
        
        let depth = grid[vertex.1][vertex.0] as i32;
        for n in get_edges(vertex, &grid) {
            let new_distance = distance + 1;
            let cost = grid[n.1][n.0] as i32 - depth;
            let is_shorter = distances.get(&n).map_or(true, |&d| new_distance < d) && cost < 2;

            if is_shorter {
                distances.insert(n, new_distance);
                to_visit.push(Visit { 
                    vertex: n,
                    distance: new_distance
                });
            }
        }
    }

    distances.get(&end).map(|u| *u)
}

fn main() {
    let input = include_str!("./../../inputs/day12.txt").trim();
    let mut grid: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let mut start = (0, 0);
    let mut end = (0, 0);
    {
        for (y, row) in grid.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                if ch == &'S' {
                    start = (x, y);
                } else if ch == &'E' {
                    end = (x, y);
                }
            }
        }
    }

    grid[start.1][start.0] = 'a';
    grid[end.1][end.0] = 'z';

    let p1 = solve_for(&grid, start, end).unwrap();
    dbg!(p1);

    let mut starts = vec![];
    {
        for (y, row) in grid.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                if ch == &'S' || ch == &'a' {
                    starts.push((x, y));
                }
            }
        }
    }
    let mut p2: Vec<usize> = starts.iter().filter_map(|&s| solve_for(&grid, s, end)).collect();
    p2.sort();
    dbg!(p2[0]);
}

use std::cmp::Ordering;

#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    distance: usize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}


use itertools::*;
use std::collections::HashSet;

struct HeightMap {
    width: i32,
    height: i32,
    data: Vec<u32>,
}

impl HeightMap {
    fn new() -> Self {
        HeightMap {
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }

    fn add_row(&mut self, row: &[u32]) {
        assert!(self.width == 0 || self.width as usize == row.len());
        if self.width == 0 {
            self.width = row.len() as i32;
        }
        self.height += 1;
        self.data.extend_from_slice(row);
    }

    fn value(&self, x: i32, y: i32) -> Option<u32> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }

        let coord = y * self.width + x;
        Some(self.data[coord as usize])
    }

    fn is_lower_than_neighbours(&self, x: i32, y: i32) -> bool {
        fn lower_than(val: u32, neighbour: Option<u32>) -> bool {
            val < neighbour.unwrap_or(u32::max_value())
        }
        let val = self.value(x, y).unwrap();

        lower_than(val, self.value(x - 1, y))
            && lower_than(val, self.value(x + 1, y))
            && lower_than(val, self.value(x, y - 1))
            && lower_than(val, self.value(x, y + 1))
    }

    fn low_points(&self) -> Vec<(i32, i32)> {
        iproduct!(0..self.width, 0..self.height)
            .filter(|(x, y)| self.is_lower_than_neighbours(*x, *y))
            .collect()
    }

    fn basin_from(&self, x: i32, y: i32) -> Basin {
        let mut res = Basin::new();
        res.add_point(x, y, self);
        res
    }
}

struct Basin {
    points: HashSet<(i32, i32)>,
}

impl Basin {
    fn new() -> Self {
        Basin {
            points: HashSet::new(),
        }
    }

    fn add_point(&mut self, x: i32, y: i32, hm: &HeightMap) {
        match hm.value(x, y) {
            Some(9) => return,
            None => return,
            _ => (),
        }

        if self.points.insert((x, y)) {
            self.add_point(x - 1, y, hm);
            self.add_point(x + 1, y, hm);
            self.add_point(x, y - 1, hm);
            self.add_point(x, y + 1, hm);
        }
    }
}

fn get_input(s: &str) -> HeightMap {
    let mut res = HeightMap::new();
    for row in s.lines() {
        let data: Vec<u32> = row.chars().map(|c| c.to_digit(10).unwrap()).collect();
        res.add_row(&data);
    }
    res
}

fn part1(hm: &HeightMap) -> u32 {
    hm.low_points()
        .into_iter()
        .map(|(x, y)| hm.value(x, y).unwrap() + 1)
        .sum()
}

fn part2(hm: &HeightMap) -> usize {
    hm.low_points()
        .into_iter()
        .map(|(x, y)| hm.basin_from(x, y))
        .map(|set| set.points.len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>()
}

fn main() {
    let test = get_input(include_str!("../input_test"));
    let input = get_input(include_str!("../input"));
    println!("test {}", part1(&test));
    println!("{}", part1(&input));
    println!("test {}", part2(&test));
    println!("{}", part2(&input));
}

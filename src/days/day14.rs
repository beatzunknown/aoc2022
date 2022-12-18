use aoc2022_lib::{utils, Day, Part};
use std::collections::HashMap;
use utils::Pos;

#[derive(Default)]
pub struct Day14 {}

impl<'a> Day<'a> for Day14 {
    fn solve(&mut self, input: &'a str, part: Part) -> String {
        let mut map: HashMap<Pos<i32>, Block> = HashMap::new();
        for line in input.lines() {
            let points: Vec<Pos<i32>> = utils::split_to_vec(line, " -> ").unwrap();
            let mut points_iter = points.into_iter();
            let mut start = points_iter.next().unwrap();
            for end in points_iter {
                for pos in start.interval_range_to(&end) {
                    map.insert(pos, Block::Rock);
                }
                start = end;
            }
        }

        let source = Pos::new(500, 0);
        let abyss_y = map.keys().map(|p| p.y()).max().unwrap();
        let floor_y = abyss_y + 2;
        'outer: loop {
            let mut sand_pos = source;
            'inner: loop {
                let old_pos = sand_pos;
                for dir in [Pos::new(0, 1), Pos::new(-1, 1), Pos::new(1, 1)] {
                    let new_pos = sand_pos + dir;
                    if matches!(map.get(&new_pos).unwrap_or(&Block::Air), Block::Air) {
                        if new_pos.y() == floor_y {
                            map.insert(new_pos, Block::Rock);
                        } else {
                            sand_pos = new_pos;
                            break;
                        }
                    }
                }
                if sand_pos == old_pos {
                    map.insert(sand_pos, Block::Sand);
                    if !part.is_first() && sand_pos == source {
                        break 'outer;
                    }
                    break 'inner;
                }
                if part.is_first() && sand_pos.y() >= abyss_y {
                    break 'outer;
                }
            }
        }

        map.values()
            .filter(|b| matches!(b, Block::Sand))
            .count()
            .to_string()
    }
}

#[derive(Default, Debug)]
enum Block {
    #[default]
    Air,
    Rock,
    Sand,
}

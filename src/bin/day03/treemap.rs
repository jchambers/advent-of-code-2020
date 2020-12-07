use std::fs::File;
use std::io;
use std::io::BufRead;

const TREE_CHAR: char = '#';

pub struct TreeMap {
    map: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl TreeMap {
    pub fn from_file(file: &File) -> TreeMap {
        TreeMap::from_lines(io::BufReader::new(file).lines()
            .filter_map(|line| line.ok()))
    }

    pub fn from_lines<I>(lines: I) -> TreeMap
    where
        I: Iterator<Item = String>,
    {
        let map: Vec<Vec<bool>> = lines.map(|line| line.chars().map(|c| c == TREE_CHAR).collect()).collect();
        let height = map.len();
        let width = map[0].len();

        TreeMap {
            map,
            width,
            height
        }
    }

    pub fn get_collisions(&self, delta_row: usize, delta_col: usize) -> u32 {
        // Don't count the starting position; start one step in
        let mut row = delta_row;
        let mut col = delta_col;
        let mut collisions = 0;

        while row < self.height {
            if self.is_tree(row, col) {
                collisions += 1;
            }

            row += delta_row;
            col += delta_col;
        }

        collisions
    }

    fn is_tree(&self, row: usize, col: usize) -> bool {
        self.map[row][col % self.width]
    }
}

#[cfg(test)]
mod test {
    use crate::treemap::TreeMap;

    #[test]
    fn from_lines() {
        let tree_map = TreeMap::from_lines(vec!["..##.......".to_string()].into_iter());

        assert_eq!(11, tree_map.width);
        assert_eq!(1, tree_map.height);
    }

    #[test]
    fn is_tree() {
        let tree_map = TreeMap::from_lines(vec!["..##.......".to_string()].into_iter());

        assert_eq!(true, tree_map.is_tree(0, 2));
        assert_eq!(true, tree_map.is_tree(0, 13));
        assert_eq!(false, tree_map.is_tree(0, 1));
    }

    #[test]
    fn get_collisions() {
        let tree_map = TreeMap::from_lines(vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string()].into_iter());

        assert_eq!(7, tree_map.get_collisions(1, 3));
    }
}
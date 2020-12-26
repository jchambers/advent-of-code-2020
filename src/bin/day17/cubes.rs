use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::cmp::{min, max};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq)]
pub struct PocketDimension {
    cells: HashSet<(isize, isize, isize)>,
}

impl PocketDimension {
    fn get_bounds(&self) -> (RangeInclusive<isize>, RangeInclusive<isize>, RangeInclusive<isize>) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        let mut min_z = 0;
        let mut max_z = 0;

        for (x, y, z) in self.cells.iter() {
            min_x = min(*x, min_x);
            max_x = max(*x, max_x);
            min_y = min(*y, min_y);
            max_y = max(*y, max_y);
            min_z = min(*z, min_z);
            max_z = max(*z, max_z);
        }

        (min_x..=max_x, min_y..=max_y, min_z..=max_z)
    }

    fn is_cell_active(&self, coordinates: (isize, isize, isize)) -> bool {
        self.cells.contains(&coordinates)
    }

    fn get_active_neighbors(&self, coordinates: (isize, isize, isize)) -> u8 {
        let mut active_neighbors = 0;
        for x  in (coordinates.0 - 1)..=(coordinates.0 + 1) {
            for y in (coordinates.1 - 1)..=(coordinates.1 + 1) {
                for z in (coordinates.2 - 1)..=(coordinates.2 + 1) {
                    if (x, y, z) != coordinates && self.is_cell_active((x, y, z)) {
                        active_neighbors += 1;
                    }
                }
            }
        }

        active_neighbors
    }

    fn into_next_round(self) -> Self {
        let mut cells = HashSet::new();
        let bounds = self.get_bounds();

        for x in (bounds.0.start() - 1)..=(bounds.0.end() + 1) {
            for y in (bounds.1.start() - 1)..=(bounds.1.end() + 1) {
                for z in (bounds.2.start() - 1)..=(bounds.2.end() + 1) {
                    let neighbor_count = self.get_active_neighbors((x, y, z));

                    if neighbor_count == 3 || (neighbor_count == 2 && self.is_cell_active((x, y, z))) {
                        cells.insert((x, y, z));
                    }
                }
            }
        }

        PocketDimension {
            cells
        }
    }

    pub fn get_active_cells_after_rounds(mut self, rounds: usize) -> usize {
        for _ in 0..rounds {
            self = self.into_next_round();
        }

        self.cells.len()
    }
}

impl From<String> for PocketDimension {

    fn from(string: String) -> Self {
        let mut cells = HashSet::new();
        let mut y = 0;

        string.split_terminator('\n')
            .rev()
            .for_each(|line| {
                let mut x = 0;

                line.chars().for_each(|c| {
                    if c == '#' {
                        cells.insert((x, y, 0));
                    }

                    x += 1;
                });

                y += 1;
            });

        PocketDimension {
            cells
        }
    }
}

impl fmt::Display for PocketDimension {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let bounds = self.get_bounds();

        for z in bounds.clone().2 {
            write!(f, "z={}\n", z)?;

            for y in bounds.clone().1.rev() {
                for x in bounds.clone().0 {
                    let c = if self.is_cell_active((x, y, z)) {
                        '#'
                    } else {
                        '.'
                    };

                    write!(f, "{}", c)?;
                }

                write!(f, "\n")?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::cubes::PocketDimension;
    use std::collections::HashSet;

    #[test]
    fn pocket_dimension_from_string() {
        let string = String::from(
            ".#.\n\
            ..#\n\
            ###\n");

        let mut expected_cells = HashSet::new();
        expected_cells.insert((0, 0, 0));
        expected_cells.insert((1, 0, 0));
        expected_cells.insert((2, 0, 0));
        expected_cells.insert((2, 1, 0));
        expected_cells.insert((1, 2, 0));

        let expected = PocketDimension {
            cells: expected_cells,
        };

        assert_eq!(expected, PocketDimension::from(string));
    }

    #[test]
    fn get_bounds() {
        let pocket_dimension = PocketDimension::from(String::from(
            "...\n\
            ..#\n\
            ###\n"));

        assert_eq!((0..=2, 0..=1, 0..=0), pocket_dimension.get_bounds());
    }

    #[test]
    fn is_cell_active() {
        let pocket_dimension = PocketDimension::from(String::from(
            ".#.\n\
            ..#\n\
            ###\n"));

        assert!(pocket_dimension.is_cell_active((0, 0, 0)));
        assert!(pocket_dimension.is_cell_active((1, 0, 0)));
        assert!(pocket_dimension.is_cell_active((2, 0, 0)));
        assert!(!pocket_dimension.is_cell_active((0, 1, 0)));
        assert!(!pocket_dimension.is_cell_active((1, 1, 0)));
        assert!(pocket_dimension.is_cell_active((2, 1, 0)));
        assert!(!pocket_dimension.is_cell_active((0, 2, 0)));
        assert!(pocket_dimension.is_cell_active((1, 2, 0)));
        assert!(!pocket_dimension.is_cell_active((2, 2, 0)));
        assert!(!pocket_dimension.is_cell_active((0, 0, 1)));
    }

    #[test]
    fn get_active_neighbors() {
        let pocket_dimension = PocketDimension::from(String::from(
            ".#.\n\
            ..#\n\
            ###\n"));

        assert_eq!(5, pocket_dimension.get_active_neighbors((1, 1, 0)));
        assert_eq!(2, pocket_dimension.get_active_neighbors((2, 0, 0)));
        assert_eq!(0, pocket_dimension.get_active_neighbors((7, -7, 7)));
    }

    #[test]
    fn get_active_cells_after_rounds() {
        let pocket_dimension = PocketDimension::from(String::from(
            ".#.\n\
            ..#\n\
            ###\n"));

        assert_eq!(112, pocket_dimension.get_active_cells_after_rounds(6));
    }
}
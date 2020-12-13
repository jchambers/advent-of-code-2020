use crate::seating::Cell::{EmptySeat, OccupiedSeat, Floor};

#[derive(Debug, Eq, PartialEq)]
pub struct SeatingMap {
    width: usize,
    height: usize,
    spaces: Vec<Cell>,
}

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat
}

impl SeatingMap {
    pub fn from_layout(layout: String) -> SeatingMap {
        let width = layout.find('\n').unwrap();
        let spaces: Vec<Cell> = layout.chars()
            .filter(|c| c != &'\n')
            .map(|c| match c {
                'L' => EmptySeat,
                '#' => OccupiedSeat,
                _ => Floor,
            }).collect();

        let height = spaces.len() / width;

        SeatingMap {
            width,
            height,
            spaces
        }
    }

    fn cell_at(&self, row: usize, col: usize) -> &Cell {
        &self.spaces[(row * self.width) + col]
    }

    fn get_adjacent_occupied_seats(&self, row: usize, col: usize) -> u8 {
        let min_row = if row == 0 { 0 } else { row - 1 };
        let max_row = if row == self.height - 1 { self.height - 1 } else { row + 1 };
        let min_col = if col == 0 { 0 } else { col - 1 };
        let max_col = if col == self.width - 1 { self.width - 1 } else { col + 1 };

        let mut occupied_neighbors = 0;

        for r in min_row..=max_row {
            for c in min_col..=max_col {
                if r != row || c != col {
                    if self.cell_at(r, c) == &OccupiedSeat {
                        occupied_neighbors += 1;
                    }
                }
            }
        }

        occupied_neighbors
    }

    fn get_visible_occupied_seats(&self, row: usize, col: usize) -> u8 {
        let mut visible_occupied_seats = 0;

        for delta_row in -1..=1 {
            for delta_col in -1..=1 {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let mut r = row as isize + delta_row;
                let mut c = col as isize + delta_col;

                while r >= 0 && r < self.height as isize && c >= 0 && c < self.width as isize {
                    match self.cell_at(r as usize, c as usize) {
                        OccupiedSeat => {
                            visible_occupied_seats += 1;
                            break;
                        },

                        EmptySeat => break,
                        Floor => {},
                    }

                    r += delta_row;
                    c += delta_col;
                }
            }
        }

        visible_occupied_seats
    }

    fn get_next_round<F>(&self, neighbor_threshold: u8, get_neighbor_fn: F) -> SeatingMap
        where F: Fn(&SeatingMap, usize, usize) -> u8
    {
        let mut spaces = Vec::with_capacity(self.spaces.len());

        for row in 0..self.height {
            for col in 0..self.width {
                let cell = match self.cell_at(row, col) {
                    Floor => Floor,
                    EmptySeat => { if get_neighbor_fn(self, row, col) == 0 { OccupiedSeat } else { EmptySeat} },
                    OccupiedSeat => { if get_neighbor_fn(self, row, col) >= neighbor_threshold { EmptySeat } else { OccupiedSeat} }
                };

                spaces.push(cell);
            }
        };

        SeatingMap {
            width: self.width,
            height: self.height,
            spaces
        }
    }

    pub fn into_stable_configuration_by_adjacency(self) -> SeatingMap {
        self.into_stable_configuration(4, &SeatingMap::get_adjacent_occupied_seats)
    }

    pub fn into_stable_configuration_by_visibility(self) -> SeatingMap {
        self.into_stable_configuration(5, &SeatingMap::get_visible_occupied_seats)
    }

    fn into_stable_configuration<F>(mut self, neighbor_threshold: u8, get_neighbor_fn: &F) -> SeatingMap
        where F: Fn(&SeatingMap, usize, usize) -> u8
    {
        loop {
            let next = self.get_next_round(neighbor_threshold, get_neighbor_fn);

            if next == self {
                return self;
            }

            self = next;
        }
    }

    pub fn get_occupied_seats(&self) -> usize {
        self.spaces.iter().filter(|cell| cell == &&OccupiedSeat).count()
    }
}

#[cfg(test)]
mod test {
    use crate::seating;
    use crate::seating::SeatingMap;
    use crate::seating::Cell::{Floor, EmptySeat, OccupiedSeat};

    #[test]
    fn from_layout() {
        let layout =
            ".LL\n\
             #.L\n";

        let seating_map = seating::SeatingMap::from_layout(String::from(layout));

        let expected = SeatingMap {
            width: 3,
            height: 2,
            spaces: vec![Floor, EmptySeat, EmptySeat, OccupiedSeat, Floor, EmptySeat],
        };

        assert_eq!(expected, seating_map);
    }

    #[test]
    fn cell_at() {
        let layout =
            ".LL\n\
             #.L\n";

        let seating_map = seating::SeatingMap::from_layout(String::from(layout));

        assert_eq!(&Floor,        seating_map.cell_at(0, 0));
        assert_eq!(&EmptySeat,    seating_map.cell_at(0, 1));
        assert_eq!(&EmptySeat,    seating_map.cell_at(0, 2));
        assert_eq!(&OccupiedSeat, seating_map.cell_at(1, 0));
        assert_eq!(&Floor,        seating_map.cell_at(1, 1));
        assert_eq!(&EmptySeat,    seating_map.cell_at(1, 2));
    }

    #[test]
    fn get_adjacent_occupied_seats() {
        let layout =
            ".LL\n\
             #.L\n";

        let seating_map = seating::SeatingMap::from_layout(String::from(layout));

        assert_eq!(1, seating_map.get_adjacent_occupied_seats(0, 0));
        assert_eq!(1, seating_map.get_adjacent_occupied_seats(0, 1));
        assert_eq!(0, seating_map.get_adjacent_occupied_seats(0, 2));
        assert_eq!(0, seating_map.get_adjacent_occupied_seats(1, 0));
        assert_eq!(1, seating_map.get_adjacent_occupied_seats(1, 1));
        assert_eq!(0, seating_map.get_adjacent_occupied_seats(1, 2));
    }

    #[test]
    fn get_next_round() {
        let initial_layout =
            "#.##.##.##\n\
             #######.##\n\
             #.#.#..#..\n\
             ####.##.##\n\
             #.##.##.##\n\
             #.#####.##\n\
             ..#.#.....\n\
             ##########\n\
             #.######.#\n\
             #.#####.##\n";

        let next_round_layout =
            "#.LL.L#.##\n\
             #LLLLLL.L#\n\
             L.L.L..L..\n\
             #LLL.LL.L#\n\
             #.LL.LL.LL\n\
             #.LLLL#.##\n\
             ..L.L.....\n\
             #LLLLLLLL#\n\
             #.LLLLLL.L\n\
             #.#LLLL.##\n";

        let initial_map = seating::SeatingMap::from_layout(String::from(initial_layout));
        let next_round_map = seating::SeatingMap::from_layout(String::from(next_round_layout));

        assert_eq!(next_round_map, initial_map.get_next_round(4, SeatingMap::get_adjacent_occupied_seats));
    }

    #[test]
    fn into_stable_configuration_by_adjacency() {
        let initial_layout =
            "#.##.##.##\n\
             #######.##\n\
             #.#.#..#..\n\
             ####.##.##\n\
             #.##.##.##\n\
             #.#####.##\n\
             ..#.#.....\n\
             ##########\n\
             #.######.#\n\
             #.#####.##\n";

        let stable_layout =
            "#.#L.L#.##\n\
             #LLL#LL.L#\n\
             L.#.L..#..\n\
             #L##.##.L#\n\
             #.#L.LL.LL\n\
             #.#L#L#.##\n\
             ..L.L.....\n\
             #L#L##L#L#\n\
             #.LLLLLL.L\n\
             #.#L#L#.##\n";

        let initial_map = seating::SeatingMap::from_layout(String::from(initial_layout));
        let stable_map = seating::SeatingMap::from_layout(String::from(stable_layout));

        assert_eq!(stable_map, initial_map.into_stable_configuration_by_adjacency());
    }

    #[test]
    fn into_stable_configuration_by_visibility() {
        let initial_layout =
            "#.##.##.##\n\
             #######.##\n\
             #.#.#..#..\n\
             ####.##.##\n\
             #.##.##.##\n\
             #.#####.##\n\
             ..#.#.....\n\
             ##########\n\
             #.######.#\n\
             #.#####.##\n";

        let stable_layout =
            "#.L#.L#.L#\n\
             #LLLLLL.LL\n\
             L.L.L..#..\n\
             ##L#.#L.L#\n\
             L.L#.LL.L#\n\
             #.LLLL#.LL\n\
             ..#.L.....\n\
             LLL###LLL#\n\
             #.LLLLL#.L\n\
             #.L#LL#.L#\n";

        let initial_map = seating::SeatingMap::from_layout(String::from(initial_layout));
        let stable_map = seating::SeatingMap::from_layout(String::from(stable_layout));

        assert_eq!(stable_map, initial_map.into_stable_configuration_by_visibility());
    }

    #[test]
    fn get_occupied_seats() {
        let layout =
            "#.#L.L#.##\n\
             #LLL#LL.L#\n\
             L.#.L..#..\n\
             #L##.##.L#\n\
             #.#L.LL.LL\n\
             #.#L#L#.##\n\
             ..L.L.....\n\
             #L#L##L#L#\n\
             #.LLLLLL.L\n\
             #.#L#L#.##\n";

        let seating_map = seating::SeatingMap::from_layout(String::from(layout));

        assert_eq!(37, seating_map.get_occupied_seats());
    }

    #[test]
    fn get_visible_occupied_seats() {
        {
            let layout =
                ".......#.\n\
                 ...#.....\n\
                 .#.......\n\
                 .........\n\
                 ..#L....#\n\
                 ....#....\n\
                 .........\n\
                 #........\n\
                 ...#.....\n";

            assert_eq!(8, seating::SeatingMap::from_layout(String::from(layout)).get_visible_occupied_seats(4, 3));
        }

        {
            let layout =
                ".............\n\
                 .L.L.#.#.#.#.\n\
                 .............\n";

            assert_eq!(0, seating::SeatingMap::from_layout(String::from(layout)).get_visible_occupied_seats(1, 1));
            assert_eq!(1, seating::SeatingMap::from_layout(String::from(layout)).get_visible_occupied_seats(1, 3));
        }

        {
            let layout =
                ".##.##.\n\
                 #.#.#.#\n\
                 ##...##\n\
                 ...L...\n\
                 ##...##\n\
                 #.#.#.#\n\
                 .##.##.\n";

            assert_eq!(0, seating::SeatingMap::from_layout(String::from(layout)).get_visible_occupied_seats(3, 3));
        }
    }
}
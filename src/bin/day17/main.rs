mod cubes;

fn main() {
    {
        let pocket_dimension = cubes::PocketDimension::from(String::from(
            ".###.###\n\
         .#.#...#\n\
         ..##.#..\n\
         ..##..##\n\
         ........\n\
         ##.#.#.#\n\
         ..###...\n\
         .####...\n"));

        println!("Cells active after 6 rounds: {}", pocket_dimension.get_active_cells_after_rounds(6));
    }

    {
        let pocket_dimension = cubes::PocketDimension::from(String::from(
            ".###.###\n\
         .#.#...#\n\
         ..##.#..\n\
         ..##..##\n\
         ........\n\
         ##.#.#.#\n\
         ..###...\n\
         .####...\n"));

        println!("Cells active after 6 rounds of hypercube evolution: {}", pocket_dimension.get_active_cells_after_rounds_hypercube(6));
    }
}
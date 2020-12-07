mod treemap;

use std::fs::File;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let tree_map = treemap::TreeMap::from_file(&file);

        let slopes: [(usize, usize); 5] = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
        let mut collision_product:u64 = 1;

        for (delta_row, delta_col) in slopes.iter() {
            let collisions = tree_map.get_collisions(*delta_row, *delta_col);
            println!("Collisions with ∆row = {}, ∆col = {}: {}", delta_row, delta_col, collisions);

            collision_product *= collisions as u64;
        }

        println!("Product of all collisions: {}", collision_product);
    } else {
        simple_error::bail!("Usage: day03 INPUT_FILE_PATH");
    }

    Ok(())
}

mod treemap;

use std::fs::File;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let tree_map = treemap::TreeMap::from_file(&file);
        println!("Collisions with ∆row = 1, ∆col = 3: {}", tree_map.get_collisions(1, 3));
    } else {
        simple_error::bail!("Usage: day03 INPUT_FILE_PATH");
    }

    Ok(())
}

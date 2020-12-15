mod count;

fn main() {
    let initial_values = vec![6, 3, 15, 13, 1, 0];
    println!("Round 2020 with initial values {:?}: {}",
             initial_values, count::play_game(&initial_values, 2020));
}
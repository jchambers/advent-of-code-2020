use std::collections::VecDeque;

mod cards;

fn main() {
    let decks = (VecDeque::from(vec![12, 40, 50, 4, 24, 15, 22, 43, 18, 21, 2, 42, 27, 36, 6, 31, 35, 20, 32, 1, 41, 14, 9, 44, 8]),
        VecDeque::from(vec![30, 10, 47, 29, 13, 11, 49, 7, 25, 37, 33, 48, 16, 5, 45, 19, 17, 26, 46, 23, 34, 39, 28, 3, 38]));

    println!("Score: {}", cards::play_combat(decks));
}
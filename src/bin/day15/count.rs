use std::collections::HashMap;

pub fn play_game(initial_values: &Vec<u64>, round: usize) -> u64 {
    assert!(!initial_values.is_empty());

    if round <= initial_values.len() {
        return initial_values[round - 1];
    }

    let mut last_occurrences = HashMap::new();
    let mut last_number_and_previous_occurrence = (0, None);

    for i in 0..initial_values.len() {
        last_number_and_previous_occurrence = (initial_values[i], last_occurrences.insert(initial_values[i], i + 1));
        // println!("Current round: {}; last number and previous occurrence: {:?}", i + 1, last_number_and_previous_occurrence);
    }

    for current_round in initial_values.len() + 1..=round {
        let age = if let Some(previous_occurrence) = last_number_and_previous_occurrence.1 {
            ((current_round - 1) - previous_occurrence) as u64
        } else {
            0
        };

        // println!("Current round: {}; last number and previous occurrence: {:?} => {}", current_round, last_number_and_previous_occurrence, age);
        last_number_and_previous_occurrence = (age, last_occurrences.insert(age, current_round));
    }

    last_number_and_previous_occurrence.0
}

#[cfg(test)]
mod test {
    use crate::count;

    #[test]
    fn play_game() {
        assert_eq!(0, count::play_game(&vec![0, 3, 6], 1));
        assert_eq!(3, count::play_game(&vec![0, 3, 6], 2));
        assert_eq!(6, count::play_game(&vec![0, 3, 6], 3));
        assert_eq!(0, count::play_game(&vec![0, 3, 6], 4));
        assert_eq!(3, count::play_game(&vec![0, 3, 6], 5));
        assert_eq!(3, count::play_game(&vec![0, 3, 6], 6));
        assert_eq!(1, count::play_game(&vec![0, 3, 6], 7));
        assert_eq!(0, count::play_game(&vec![0, 3, 6], 8));
        assert_eq!(4, count::play_game(&vec![0, 3, 6], 9));
        assert_eq!(0, count::play_game(&vec![0, 3, 6], 10));

        assert_eq!(1, count::play_game(&vec![1, 3, 2], 2020));
        assert_eq!(10, count::play_game(&vec![2, 1, 3], 2020));
        assert_eq!(27, count::play_game(&vec![1, 2, 3], 2020));
        assert_eq!(78, count::play_game(&vec![2, 3, 1], 2020));
        assert_eq!(438, count::play_game(&vec![3, 2, 1], 2020));
        assert_eq!(1836, count::play_game(&vec![3, 1, 2], 2020));
    }
}

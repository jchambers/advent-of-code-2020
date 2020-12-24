use std::collections::VecDeque;

pub fn play_combat(mut decks: (VecDeque<u32>, VecDeque<u32>)) -> u32 {
    loop {
        if decks.0.is_empty() {
            return get_combat_score(&decks.1);
        } else if decks.1.is_empty() {
            return get_combat_score(&decks.0);
        }

        let first = decks.0.pop_front().unwrap();
        let second = decks.1.pop_front().unwrap();

        if first > second {
            decks.0.push_back(first);
            decks.0.push_back(second);
        } else {
            decks.1.push_back(second);
            decks.1.push_back(first);
        }
    }
}

fn get_combat_score(deck: &VecDeque<u32>) -> u32 {
    deck.iter().zip((1..=deck.len()).rev())
        .map(|(card, position)| card * position as u32)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::cards;
    use std::collections::VecDeque;

    #[test]
    fn get_combat_score() {
        assert_eq!(306, cards::get_combat_score(&VecDeque::from(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1])));
    }

    #[test]
    fn play_combat() {
        assert_eq!(306, cards::play_combat((VecDeque::from(vec![9, 2, 6, 3, 1]),
                                            VecDeque::from(vec![5, 8, 4, 7, 10]))));
    }
}
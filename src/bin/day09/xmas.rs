pub fn find_outlier(numbers: &[i64], window_size: usize) -> Option<i64> {
    assert!(numbers.len() > window_size);

    for offset in 0..(numbers.len() - window_size) {
        let candidate = numbers[offset + window_size];

        let mut slice = numbers[offset..(offset + window_size)].to_vec();
        slice.sort();

        if ksum::ksum(slice.as_slice(), 2, candidate).is_empty() {
            return Some(candidate)
        }
    }

    None
}

pub fn find_consecutive_sum(numbers: &[i64], target: i64) -> Option<&[i64]> {
    assert!(numbers.len() > 1);

    for window_size in 2..numbers.len() {
        for offset in 0..(numbers.len() - window_size) {
            if target == numbers[offset..(offset + window_size)].iter().sum() {
                return Some(&numbers[offset..(offset + window_size)]);
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::xmas;

    #[test]
    fn find_outliet() {
        let numbers = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];

        assert_eq!(Some(127), xmas::find_outlier(&numbers, 5));
    }

    #[test]
    fn find_consecutive_sum() {
        let numbers = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];

        assert_eq!(Some(&numbers[2..6]), xmas::find_consecutive_sum(&numbers, 127));
    }
}
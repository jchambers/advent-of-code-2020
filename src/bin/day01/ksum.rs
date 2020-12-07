use std::collections::HashSet;

pub fn ksum(numbers: &[i32], k: usize, target: i32) -> HashSet<Vec<i32>> {
    assert!(k > 0, "K must be positive");
    assert!(numbers.len() >= k, "Number list must be longer than k");

    let mut solutions = HashSet::new();

    if k == 1 {
        for n in numbers {
            if *n == target {
                solutions.insert(vec![*n]);
            }
        }
    } else if k == 2 {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            if numbers[left] + numbers[right] == target {
                solutions.insert(vec![numbers[left], numbers[right]]);

                advance_left(numbers, &mut left);
                advance_right(numbers, &mut right);
            } else if numbers[left] + numbers[right] < target {
                advance_left(numbers, &mut left);
            } else {
                advance_right(numbers, &mut right);
            }
        }
    } else {
        for i in 0..=(numbers.len() - k) {
            for mut solution in ksum(&numbers[(i + 1)..], k - 1, target - numbers[i]) {
                solution.insert(0, numbers[i]);
                solutions.insert(solution);
            }
        }
    }

    solutions
}

fn advance_left(numbers: &[i32], left: &mut usize) {
    let initial_left_value = numbers[*left];

    while numbers[*left] == initial_left_value && *left < numbers.len() {
        *left += 1;
    }
}

fn advance_right(numbers: &[i32], right: &mut usize) {
    let initial_right_value = numbers[*right];

    while numbers[*right] == initial_right_value && *right != 0 {
        *right -= 1;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::ksum::ksum;

    #[test]
    #[should_panic]
    fn non_positive_k() {
        ksum(vec![1, 2, 3].as_mut(), 0, 0);
    }

    #[test]
    #[should_panic]
    fn undersized_vec_panic() {
        ksum(vec![1].as_mut(), 2, 0);
    }

    #[test]
    fn one_sum() {
        {
            let mut expected = HashSet::new();
            expected.insert(vec![4]);

            assert_eq!(expected, ksum(&[1, 2, 2, 2, 3, 4], 1, 4));
        }

        assert_eq!(0, ksum(&[1, 2, 2, 2, 3, 4], 1, 5).len());
    }

    #[test]
    fn two_sum() {
        {
            let mut expected = HashSet::new();
            expected.insert(vec![1, 4]);
            expected.insert(vec![2, 3]);

            assert_eq!(expected, ksum(&[1, 2, 2, 2, 3, 4], 2, 5));
        }

        {
            let mut expected = HashSet::new();
            expected.insert(vec![1, 2]);

            assert_eq!(expected, ksum(&[1, 2], 2, 3));
        }

        assert_eq!(0, ksum(&[1, 2, 2, 2, 3, 4], 2, 12).len());
    }

    #[test]
    fn three_sum() {
        {
            let mut expected = HashSet::new();
            expected.insert(vec![1, 1, 3]);
            expected.insert(vec![1, 2, 2]);

            assert_eq!(expected, ksum(&[1, 1, 2, 2, 3], 3, 5));
        }

        {
            let mut expected = HashSet::new();
            expected.insert(vec![1, 2, 3]);

            assert_eq!(expected, ksum(&[1, 2, 3], 3, 6));
        }

        assert_eq!(0, ksum(&[1, 1, 2, 2, 3], 3, 12).len());
    }
}

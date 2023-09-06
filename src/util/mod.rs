mod combinations;
mod permutations;

pub use combinations::*;
pub use permutations::*;

use std::cmp::{Ordering, Reverse};

pub fn filter_2d_vector<F, T>(arrays: &mut [Vec<T>], is_delete: F)
where
    F: Fn(usize, &T) -> bool,
{
    for (index, value) in arrays.iter_mut().enumerate() {
        value.retain(|item| !is_delete(index, item));
    }
}

pub fn sort_2d_vector<F, T>(arrays: &mut [Vec<T>], compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    for value in arrays {
        value.sort_by(&compare);
    }
}
pub fn get_threshold<F, K, T, const LEN: usize>(
    arrays: &[Vec<T>; LEN],
    limit_index: usize,
    reverse: bool,
    f: F,
) -> [K; LEN]
where
    F: Fn(&T) -> K,
    K: Ord,
{
    let mut threshold: [K; LEN] = unsafe { std::mem::zeroed() };

    for (index, value) in arrays.iter().enumerate() {
        let mut sorted_values: Vec<&T> = value.iter().collect();
        if reverse {
            sorted_values.sort_by_key(|v| Reverse(f(v)));
        } else {
            sorted_values.sort_by_key(|v| f(v));
        }

        threshold[index] = match sorted_values.get(limit_index) {
            Some(value) => f(value),
            None => f(sorted_values.last().unwrap()),
        };
    }

    threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_min_threshold_works() {
        let array: [Vec<i32>; 3] = [
            vec![1, 2, 3, -4, 5, 6, 7, 8],
            vec![1, 2, 3, -4],
            vec![1, 2, 3],
        ];
        let a = get_threshold(&array, 2, true, |v| v.abs());
        let b = [6, 2, 1];
        assert_eq!(a, b);
    }
    #[test]
    fn filter_2d_vector_works() {
        let mut a: [Vec<i32>; 3] = [
            vec![1, 2, 3, -4, 5, 6, 7, 8],
            vec![1, 2, 3, -4],
            vec![1, 2, 3],
        ];

        filter_2d_vector(&mut a, |_, v| v.abs() < 4);
        let b: [Vec<i32>; 3] = [vec![-4, 5, 6, 7, 8], vec![-4], vec![]];
        assert_eq!(a, b);
    }
}

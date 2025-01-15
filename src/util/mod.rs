mod combinations;
mod permutations;

pub use combinations::*;
pub use permutations::*;

use std::cmp::{Ordering, Reverse};

/// Filter a 2D vector
///
/// # Arguments
///
/// - `arrays` - A mutable reference to a 2D vector
/// - `is_delete` - A closure that takes an index and a value and returns a boolean
///
/// # Example
///
/// ```rust
/// use util::filter_2d_vector;
///
/// let mut a: [Vec<i32>; 3] = [
///     vec![1, 2, 3, -4, 5, 6, 7, 8],
///     vec![1, 2, 3, -4],
///     vec![1, 2, 3],
/// ];
/// filter_2d_vector(&mut a, |_, v| v.abs() < 4);
/// let b: [Vec<i32>; 3] = [vec![-4, 5, 6, 7, 8], vec![-4], vec![]];
/// assert_eq!(a, b);
/// ```
///
/// # Panics
///
/// This function will panic if the length of the 2D vector is zero
pub fn filter_2d_vector<F, T>(arrays: &mut [Vec<T>], is_delete: F)
where
    F: Fn(usize, &T) -> bool,
{
    for (index, value) in arrays.iter_mut().enumerate() {
        value.retain(|item| !is_delete(index, item));
    }
}

/// Sort a 2D vector
///
/// # Arguments
///
/// - `arrays` - A mutable reference to a 2D vector
/// - `compare` - A closure that takes two values and returns an ordering
///
/// # Example
///
/// ```rust
/// use util::sort_2d_vector;
///
/// let mut a: [Vec<i32>; 3] = [
///    vec![1, 2, 3, -4, 5, 6, 7, 8],
///   vec![1, 2, 3, -4],
///  vec![1, 2, 3],
/// ];
///
/// sort_2d_vector(&mut a, |a, b| a.cmp(b));
/// let b: [Vec<i32>; 3] = [
///    vec![-4, 1, 2, 3, 5, 6, 7, 8],
///   vec![-4, 1, 2, 3],
/// vec![1, 2, 3],
/// ];
///
/// assert_eq!(a, b);
/// ```
///
/// # Panics
///
/// This function will panic if the length of the 2D vector is zero
pub fn sort_2d_vector<F, T>(arrays: &mut [Vec<T>], compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    for value in arrays {
        value.sort_by(&compare);
    }
}

/// Get the threshold of a 2D vector
///
/// # Arguments
///
/// - `arrays` - A reference to a 2D vector
/// - `limit_index` - The index of the threshold
/// - `reverse` - A boolean to reverse the order
/// - `f` - A closure that takes a value and returns a key
///
/// # Returns
///
/// An array of thresholds
///
/// # Example
///
/// ```rust
/// use util::get_threshold;
///
/// let array: [Vec<i32>; 3] = [
///     vec![1, 2, 3, -4, 5, 6, 7, 8],
///     vec![1, 2, 3, -4],
///     vec![1, 2, 3],
/// ];
/// let a = get_threshold(&array, 2, true, |v| v.abs());
/// let b = [6, 2, 1];
/// assert_eq!(a, b);
/// ```
///
/// # Panics
///
/// This function will panic if the length of the 2D vector is zero
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
pub fn tarjans_scc<const LEN: usize>(adjacency_matrix: &[[bool; LEN]; LEN]) -> Vec<Vec<usize>> {
    let mut index = 0;
    let mut stack = Vec::new();
    let mut indices = [-1; LEN];
    let mut low_link = [-1; LEN];
    let mut on_stack = [false; LEN];
    let mut sccs = Vec::new();

    fn strong_connect<const LEN: usize>(
        v: usize,
        index: &mut i32,
        stack: &mut Vec<usize>,
        indices: &mut [i32],
        low_link: &mut [i32],
        on_stack: &mut [bool],
        sccs: &mut Vec<Vec<usize>>,
        adjacency_matrix: &[[bool; LEN]; LEN],
    ) {
        indices[v] = *index;
        low_link[v] = *index;
        *index += 1;
        stack.push(v);
        on_stack[v] = true;

        for neighbor in 0..LEN {
            if adjacency_matrix[v][neighbor] {
                if indices[neighbor] == -1 {
                    strong_connect(
                        neighbor,
                        index,
                        stack,
                        indices,
                        low_link,
                        on_stack,
                        sccs,
                        adjacency_matrix,
                    );
                    low_link[v] = low_link[v].min(low_link[neighbor]);
                } else if on_stack[neighbor] {
                    low_link[v] = low_link[v].min(indices[neighbor]);
                }
            }
        }

        if low_link[v] == indices[v] {
            let mut scc = Vec::new();
            while let Some(w) = stack.pop() {
                on_stack[w] = false;
                scc.push(w);
                if w == v {
                    break;
                }
            }
            sccs.push(scc);
        }
    }

    for v in 0..LEN {
        if indices[v] == -1 {
            strong_connect(
                v,
                &mut index,
                &mut stack,
                &mut indices,
                &mut low_link,
                &mut on_stack,
                &mut sccs,
                adjacency_matrix,
            );
        }
    }

    sccs
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

use std::collections::VecDeque;

pub fn next_permutation<T: Ord>(arr: &mut [T]) -> bool {
    let mut i = arr.len() - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }

    let mut j = arr.len() - 1;
    while arr[j] <= arr[i - 1] {
        j -= 1;
    }

    arr.swap(i - 1, j);
    arr[i..].reverse();
    true
}

/// Generates the next permutation of pointers in the given slice.
///
/// This function modifies the input slice to produce the next permutation
/// of its elements in ascending order. It compares the pointers directly using
/// their memory addresses.
///
/// # Arguments
///
/// * `arr` - A mutable slice of pointers to modify in-place.
///
/// # Returns
///
/// * `true` if a new permutation was generated.
/// * `false` if the input was already the last possible permutation.
///
/// # Examples
///
/// ```
/// let mut arr = [&1, &2, &3];
/// assert_eq!(next_permutation_ptr(&mut arr), true);
/// assert_eq!(arr, [&1, &3, &2]);
/// ```
pub fn next_permutation_ptr<T: ?Sized>(arr: &mut [&T]) -> bool {
    let mut i = arr.len() - 1;
    while i > 0 && (arr[i - 1] as *const T).addr() >= (arr[i] as *const T).addr() {
        i -= 1;
    }
    if i == 0 {
        return false;
    }

    let mut j = arr.len() - 1;
    while (arr[j] as *const T).addr() <= (arr[i - 1] as *const T).addr() {
        j -= 1;
    }

    arr.swap(i - 1, j);
    arr[i..].reverse();
    true
}

/// Perform breadth-first search (BFS) with pruning.
///
/// # Arguments
/// - `array`: The input array to generate permutations from.
/// - `pruning_fn`: The pruning function to decide if a branch should continue.
///
/// # Returns
/// A vector of all valid permutations after applying the pruning.
pub fn bfs_permutation_with_prune<'a, T, V, FN, const LEN: usize>(
    array: &'a [&'a V; LEN],
    initial_context: T,
    mut pruning_fn: FN,
) -> Vec<([&'a V; LEN], T)>
where
    T: Clone,
    V: ?Sized,
    FN: FnMut(T, usize, &'a V) -> Option<T>,
{
    let mut results = Vec::new();
    let mut queue: VecDeque<([Option<&V>; LEN], usize, T)> = VecDeque::new();

    // Initialize the queue with an empty permutation and initial context.
    queue.push_back(([None; LEN], 0, initial_context.clone()));

    while let Some((current_perm, perm_len, context)) = queue.pop_front() {
        // If the permutation is complete, add it to the results.
        if perm_len == LEN {
            let mut completed_perm = [array[0]; LEN]; // Temporary array
            for (i, item) in current_perm.iter().enumerate() {
                if let Some(value) = item {
                    completed_perm[i] = &value;
                }
            }
            results.push((completed_perm, context));
            continue;
        }

        for &item in array {
            // Skip items already in the current permutation to avoid duplicates.
            let item_ptr = (item as *const V).addr();
            let mut found = false;
            for &perm in &current_perm {
                if let Some(val) = perm {
                    if (val as *const V).addr() == item_ptr {
                        found = true;
                    }
                }
            }
            if found {
                continue;
            }

            // Create the next permutation array.
            let mut next_perm = current_perm;
            next_perm[perm_len] = Some(item);

            // Call the pruning function to decide whether to continue.
            if let Some(updated_context) = pruning_fn(context.clone(), perm_len, item) {
                queue.push_back((next_perm, perm_len + 1, updated_context));
            }
        }
    }

    results
}
pub fn permutation_2d<VALUE, CONTEXT, FN>(
    arr: &[Vec<&VALUE>],
    init: CONTEXT,
    mut compute: FN,
) -> Vec<CONTEXT>
where
    VALUE: ?Sized,
    CONTEXT: Clone,
    FN: FnMut(CONTEXT, &VALUE) -> CONTEXT,
{
    let mut results = vec![init];

    for sub_array in arr {
        let mut new_results = Vec::new();

        for current_context in results {
            let mut permutation = sub_array.clone();
            loop {
                let mut temp_context = current_context.clone();
                for value in &permutation {
                    temp_context = compute(temp_context, &value);
                }
                new_results.push(temp_context);
                if !next_permutation_ptr(&mut permutation) {
                    break;
                }
            }
        }

        results = new_results;
    }

    results
}
pub fn permutation_2d_usize<CONTEXT, FN>(
    arr: &[Vec<usize>],
    init: CONTEXT,
    mut compute: FN,
) -> Vec<CONTEXT>
where
    CONTEXT: Clone,
    FN: FnMut(CONTEXT, usize) -> CONTEXT,
{
    let mut results = vec![init];

    for sub_array in arr {
        let mut new_results = Vec::new();

        for current_context in results {
            let mut permutation: Vec<&usize> = sub_array.iter().map(|v| v).collect();
            loop {
                let mut temp_context = current_context.clone();
                for value in &permutation {
                    temp_context = compute(temp_context, **value);
                }
                new_results.push(temp_context);
                if !next_permutation_ptr(&mut permutation) {
                    break;
                }
            }
        }

        results = new_results;
    }

    results
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn next_permutation_works() {
        let want = [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ];

        let mut array = [0, 1, 2];
        let mut i = 0;
        loop {
            assert_eq!(array, want[i]);
            i += 1;
            if !next_permutation(&mut array) {
                break;
            }
        }

        let mut array: [&str; 3] = ["apple", "banana", "cherry"];
        let want = [
            [array[0], array[1], array[2]],
            [array[0], array[2], array[1]],
            [array[1], array[0], array[2]],
            [array[1], array[2], array[0]],
            [array[2], array[0], array[1]],
            [array[2], array[1], array[0]],
        ];

        let mut i = 0;
        loop {
            assert_eq!(array, want[i]);
            i += 1;
            if !next_permutation(&mut array) {
                break;
            }
        }
    }
    #[test]
    fn next_permutation_ptr_works() {
        let mut array: [&str; 3] = ["apple", "banana", "cherry"];
        let want = [
            [array[0], array[1], array[2]],
            [array[0], array[2], array[1]],
            [array[1], array[0], array[2]],
            [array[1], array[2], array[0]],
            [array[2], array[0], array[1]],
            [array[2], array[1], array[0]],
        ];

        let mut i = 0;
        loop {
            assert_eq!(array, want[i]);
            i += 1;
            if !next_permutation_ptr(&mut array) {
                break;
            }
        }
    }
    #[test]
    fn permutation_2d_works() {
        let array = vec![vec!["apple"], vec!["banana", "orange"], vec!["cherry"]];

        let init = "".to_string();
        let compute = |accumulated: String, next: &str| format!("{} {}", accumulated, next);
        let result = permutation_2d(&array, init, compute);

        let want = [
            " apple banana orange cherry".to_string(),
            " apple orange banana cherry".to_string(),
        ];
        for i in 0..result.len() {
            assert_eq!(result[i], want[i]);
        }
    }
    #[test]
    fn permutation_2d_usize_works() {
        let array = vec![vec![1], vec![2], vec![3, 4, 5], vec![6]];

        let init = "".to_string();
        let mut i = 0;
        let want = [
            " 1".to_string(),
            " 1 2".to_string(),
            " 1 2 3".to_string(),
            " 1 2 3 4".to_string(),
            " 1 2 3 4 5".to_string(),
            " 1 2 3".to_string(),
            " 1 2 3 5".to_string(),
            " 1 2 3 5 4".to_string(),
            " 1 2 4".to_string(),
            " 1 2 4 3".to_string(),
            " 1 2 4 3 5".to_string(),
            " 1 2 4".to_string(),
            " 1 2 4 5".to_string(),
            " 1 2 4 5 3".to_string(),
            " 1 2 5".to_string(),
            " 1 2 5 3".to_string(),
            " 1 2 5 3 4".to_string(),
            " 1 2 5".to_string(),
            " 1 2 5 4".to_string(),
            " 1 2 5 4 3".to_string(),
            " 1 2 3 4 5 6".to_string(),
            " 1 2 3 5 4 6".to_string(),
            " 1 2 4 3 5 6".to_string(),
            " 1 2 4 5 3 6".to_string(),
            " 1 2 5 3 4 6".to_string(),
            " 1 2 5 4 3 6".to_string(),
        ];
        let compute = |accumulated: String, next: usize| {
            let context = format!("{} {}", accumulated, next);
            assert_eq!(context, want[i]);
            i += 1;
            context
        };
        let result = permutation_2d_usize(&array, init, compute);

        let want = [
            " 1 2 3 4 5 6".to_string(),
            " 1 2 3 5 4 6".to_string(),
            " 1 2 4 3 5 6".to_string(),
            " 1 2 4 5 3 6".to_string(),
            " 1 2 5 3 4 6".to_string(),
            " 1 2 5 4 3 6".to_string(),
        ];
        for i in 0..result.len() {
            assert_eq!(result[i], want[i]);
        }
    }

    #[test]
    fn bfs_permutation_with_prune_works() {
        let array: [&usize; 3] = [&0, &1, &2];
        let initial_context = (vec![], 0);

        let want = vec![
            vec![0],
            vec![1],
            vec![2],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![1, 2],
            vec![2, 0],
            vec![2, 1],
            vec![0, 1, 2],
            vec![0, 2, 1],
            vec![1, 0, 2],
            vec![1, 2, 0],
            vec![2, 0, 1],
            vec![2, 1, 0],
        ];
        let mut index: usize = 0;
        let example_pruning = |mut context: (Vec<usize>, usize),
                               _depth: usize,
                               item: &usize|
         -> Option<(Vec<usize>, usize)> {
            context.0.push(*item);
            context.1 += *item;

            assert_eq!(context.0.as_slice(), &want[index]);
            index += 1;
            Some(context)
        };

        let results = bfs_permutation_with_prune(&array, initial_context, example_pruning);

        let want: [[&usize; 3]; 6] = [
            [&0, &1, &2],
            [&0, &2, &1],
            [&1, &0, &2],
            [&1, &2, &0],
            [&2, &0, &1],
            [&2, &1, &0],
        ];

        for i in 0..want.len() {
            assert_eq!(results[i].0.as_slice(), want[i].as_slice());
        }
    }
}

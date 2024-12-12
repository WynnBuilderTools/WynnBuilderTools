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

        let mut array: [usize; 3] = std::array::from_fn(|i| i);
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
}

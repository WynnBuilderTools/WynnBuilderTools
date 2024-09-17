pub struct Permutations<T, const LEN: usize> {
    array: [T; LEN],
    first_permutation: bool,
}

impl<T, const LEN: usize> Permutations<T, LEN> {
    pub fn new(array: [T; LEN]) -> Self {
        Permutations {
            array,
            first_permutation: true,
        }
    }
}

impl<T, const LEN: usize> Iterator for Permutations<T, LEN>
where
    T: Ord + Clone,
{
    type Item = [T; LEN];

    fn next(&mut self) -> Option<Self::Item>
    where
        T: Ord + Clone,
    {
        if self.first_permutation {
            self.first_permutation = false;
            return Some(self.array.clone());
        }

        let mut i = LEN - 1;
        while i > 0 && self.array[i - 1] >= self.array[i] {
            i -= 1;
        }

        if i == 0 {
            return None;
        }

        let mut j = LEN - 1;
        while self.array[j] <= self.array[i - 1] {
            j -= 1;
        }

        self.array.swap(i - 1, j);
        self.array[i..].reverse();

        Some(&self.array).cloned()
    }
}

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

pub fn next_permutation_ptr<T: ?Sized>(arr: &mut [&T]) -> bool {
    let mut i = arr.len() - 1;
    while i > 0 && !std::ptr::eq(arr[i - 1], arr[i]) {
        i -= 1;
    }
    if i == 0 {
        return false;
    }

    let mut j = arr.len() - 1;
    while !std::ptr::eq(arr[i - 1], arr[i]) {
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
    fn permutations_works() {
        let want = [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ];
        let array: [usize; 3] = std::array::from_fn(|i| i);
        for (i, v) in Permutations::new(array).enumerate() {
            assert_eq!(v, want[i]);
        }
    }
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

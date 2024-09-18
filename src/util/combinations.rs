use std::{
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use rayon::prelude::{ParallelBridge, ParallelIterator};

/// Generates all possible combinations of elements from multiple arrays and applies a function to each combination.
///
/// This function generates all possible combinations of elements from the provided arrays and applies
/// the given function `func` to each combination. The combinations are generated in parallel using the
/// Rayon library to improve performance.
///
/// # Type Parameters
///
/// - `T`: Type of the elements in the arrays.
/// - `TR`: Type that can be referenced as `T`.
/// - `F`: Type of the function to be applied to each combination.
/// - `LEN`: Length of the `arrays` array.
///
/// # Parameters
///
/// - `segment_size`: The size of each segment for generating random numbers.
/// - `count`: An `Arc<AtomicUsize>` used to keep track of the number of combinations processed.
/// - `arrays`: A reference to an array of slices, where each slice contains elements to be combined.
/// - `func`: A function to be applied to each combination of elements.
///
/// # Example
///
/// ```rust
/// use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
///
/// let arrays: [&[i32]; 3] = [&[1, 2], &[3, 4], &[5, 6]];
/// let count = Arc::new(AtomicUsize::new(0));
///
/// generate_full_combinations_with_random(10, count.clone(), &arrays, |combination| {
///     println!("{:?}", combination);
/// });
/// ```
///
/// # Panics
///
/// This function will not panic as long as the provided arrays and segment size are valid.
///
/// # Errors
///
/// This function does not return errors but may cause undefined behavior if the safety
/// requirements of the `select_from_arrays` function are not upheld.
pub fn generate_full_combinations_with_random<T, TR, F, const LEN: usize>(
    segment_size: usize,
    count: Arc<AtomicUsize>,
    arrays: &[&[TR]; LEN],
    func: F,
    combinations_counter: Option<Arc<AtomicUsize>>,
) where
    TR: Sync + AsRef<T>,
    F: Fn([&T; LEN]) + Sync,
{
    let max_indexes: [usize; LEN] = arrays.map(|f| f.len());
    let total_combinations = max_indexes.iter().product::<usize>();

    segmented_random_numbers(
        total_combinations,
        segment_size,
        count,
        combinations_counter,
    )
    .par_bridge()
    .for_each(|i| {
        let index_combinations = map_to_index_space(&max_indexes, i);
        func(unsafe { select_from_arrays(&index_combinations, arrays) });
    })
}

/// Selects elements from multiple arrays based on provided indexes.
///
/// # Safety
///
/// This function is unsafe because it uses unchecked indexing and assumes that the provided
/// indexes are within bounds of the arrays. The caller must ensure that:
/// - Each index in `indexes` is valid for the corresponding array in `arrays`.
/// - The arrays in `arrays` contain elements that can be safely referenced as `&T`.
///
/// # Type Parameters
///
/// - `'a`: Lifetime of the references in the arrays.
/// - `T`: Type of the elements to be selected.
/// - `TR`: Type that can be referenced as `T`.
/// - `ATR`: Type that can be referenced as a slice of `TR`.
/// - `LEN`: Length of the `indexes` and `arrays` arrays.
///
/// # Parameters
///
/// - `indexes`: A reference to an array of indexes used to select elements from the arrays.
/// - `arrays`: A reference to an array of arrays from which elements are selected.
///
/// # Returns
///
/// An array of references to the selected elements.
///
/// # Example
///
/// ```rust
/// let indexes = [0, 1, 2];
/// let arrays = [
///     vec![1, 2, 3],
///     vec![4, 5, 6],
///     vec![7, 8, 9],
/// ];
/// let result = unsafe { select_from_arrays(&indexes, &arrays) };
/// assert_eq!(result, [&1, &5, &9]);
/// ```
///
/// # Panics
///
/// This function will not panic as long as the caller ensures the safety requirements are met.
///
/// # Errors
///
/// This function does not return errors but may cause undefined behavior if the safety
/// requirements are not upheld.
pub unsafe fn select_from_arrays<'a, T, TR, ATR, const LEN: usize>(
    indexes: &[usize; LEN],
    arrays: &'a [ATR; LEN],
) -> [&'a T; LEN]
where
    T: 'a,
    TR: 'a + AsRef<T>,
    ATR: 'a + AsRef<[TR]>,
{
    let mut result: [MaybeUninit<&'a T>; LEN] = MaybeUninit::uninit_array();
    for i in 0..LEN {
        result[i].write(arrays[i].as_ref()[*indexes.get_unchecked(i)].as_ref());
    }
    MaybeUninit::array_assume_init(result)
}

/// Maps a linear index to a multi-dimensional index space.
///
/// This function converts a single linear index into a multi-dimensional index
/// based on the provided maximum sizes for each dimension. It is useful for
/// translating a flat array index into coordinates for a multi-dimensional array.
///
/// # Type Parameters
///
/// - `LEN`: The number of dimensions.
///
/// # Parameters
///
/// - `array_max`: A reference to an array containing the maximum sizes for each dimension.
/// - `index`: The linear index to be converted.
///
/// # Returns
///
/// An array of indexes representing the multi-dimensional coordinates.
///
/// # Example
///
/// ```rust
/// let array_max = [3, 4, 5];
/// let index = 23;
/// let result = map_to_index_space(&array_max, index);
/// assert_eq!(result, [1, 1, 3]);
/// ```
///
/// ```rust
/// let array_max = [3, 3, 3];
/// let index = 1;
/// let result = map_to_index_space(&array_max, index);
/// assert_eq!(result, [0, 0, 1]);
/// ```
///
/// # Panics
///
/// This function does not panic as long as the `index` is within the bounds defined by `array_max`.
pub fn map_to_index_space<const LEN: usize>(
    array_max: &[usize; LEN],
    index: usize,
) -> [usize; LEN] {
    let mut result = [Default::default(); LEN];
    let mut remaining = index;
    for i in (0..LEN).rev() {
        let current = remaining % array_max[i];
        result[i] = current;
        remaining /= array_max[i];
    }
    result
}

pub fn segmented_random_numbers(
    max: usize,
    segment_size: usize,
    count: Arc<AtomicUsize>,
    combinations_counter: Option<Arc<AtomicUsize>>,
) -> impl Iterator<Item = usize> {
    /// A struct for generating segmented random numbers.
    ///
    /// `SegmentedRandomNumbers` is used to generate random numbers in segments. It maintains
    /// the state of the current segment and index, and provides methods to select the next
    /// index within the current segment or the last segment.
    ///
    /// # Fields
    ///
    /// - `rng`: A random number generator from the `fastrand` crate.
    /// - `segments`: A vector of segment base indexes.
    /// - `segment_size`: The size of each segment.
    /// - `current_segment`: The index of the current segment.
    /// - `current_index`: The current index within the current segment.
    /// - `count`: An `Arc<AtomicUsize>` used to keep track of the number of random numbers generated.
    /// - `last_segments_size`: An optional size for the last segment.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
    ///
    /// let segments = vec![0, 10, 20];
    /// let segment_size = 10;
    /// let count = Arc::new(AtomicUsize::new(0));
    ///
    /// let mut srn = SegmentedRandomNumbers {
    ///     rng: fastrand::Rng::default(),
    ///     segments,
    ///     segment_size,
    ///     current_segment: 0,
    ///     current_index: 0,
    ///     count: count.clone(),
    ///     last_segments_size: Some(5),
    /// };
    ///
    /// while let Some(index) = srn.next() {
    ///     println!("Generated index: {}", index);
    /// }
    /// ```
    struct SegmentedRandomNumbers {
        rng: fastrand::Rng,
        segments: Vec<usize>,
        segment_size: usize,
        current_segment: usize,
        current_index: usize,
        count: Arc<AtomicUsize>,
        last_segments_size: Option<usize>,
        combinations_counter: Option<Arc<AtomicUsize>>,
    }

    impl Iterator for SegmentedRandomNumbers {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.count.fetch_add(1, Ordering::AcqRel);
            match &(self.combinations_counter) {
                Some(counter) => {
                    counter.fetch_sub(1, Ordering::AcqRel);
                }
                None => {}
            }

            if let Some(last_size) = self.last_segments_size {
                if self.current_index < last_size {
                    return Some(self.select_in_last_segment());
                } else {
                    self.current_index = 0;
                    self.last_segments_size = None;
                }
            }

            if self.segments.is_empty() {
                None
            } else if self.current_index < self.segment_size {
                Some(self.select_in_segment())
            } else {
                // fast remove
                // it same self.segments.remove(self.current_segment);
                let last = self.segments.len() - 1;
                self.segments.swap(self.current_segment, last);
                self.segments.pop();

                if self.segments.is_empty() {
                    None
                } else {
                    self.current_index = 0;
                    self.current_segment = self.rng.usize(0..self.segments.len());
                    Some(self.select_in_segment())
                }
            }
        }
    }
    impl SegmentedRandomNumbers {
        /// Selects the next index within the current segment.
        ///
        /// This function calculates the next index within the current segment by adding the current
        /// segment's base index to the current index within the segment. It then increments the
        /// current index for the next call.
        ///
        /// # Returns
        ///
        /// The next index within the current segment.
        ///
        /// # Example
        ///
        /// ```rust
        /// let mut srn = SegmentedRandomNumbers {
        ///     segments: vec![0, 10, 20],
        ///     current_segment: 1,
        ///     current_index: 0,
        ///     segment_size: 10,
        ///     rng: fastrand::Rng::default(),
        ///     last_segments_size: None,
        /// };
        /// let index = srn.select_in_segment();
        /// assert_eq!(index, 10);
        /// ```
        fn select_in_segment(&mut self) -> usize {
            let result = self.segments[self.current_segment] + self.current_index;
            self.current_index += 1;
            result
        }

        /// Selects the next index within the last segment.
        ///
        /// This function calculates the next index within the last segment by adding the total number
        /// of segments multiplied by the segment size to the current index within the last segment.
        /// It then increments the current index for the next call.
        ///
        /// # Returns
        ///
        /// The next index within the last segment.
        ///
        /// # Example
        ///
        /// ```rust
        /// let mut srn = SegmentedRandomNumbers {
        ///     segments: vec![0, 10, 20],
        ///     current_segment: 2,
        ///     current_index: 0,
        ///     segment_size: 10,
        ///     rng: fastrand::Rng::default(),
        ///     last_segments_size: Some(5),
        /// };
        /// let index = srn.select_in_last_segment();
        /// assert_eq!(index, 30);
        /// ```
        fn select_in_last_segment(&mut self) -> usize {
            let result = self.segments.len() * self.segment_size + self.current_index;
            self.current_index += 1;
            result
        }
    }

    let rng = fastrand::Rng::default();
    let segment_count = (max + 1) / segment_size;
    let segments = (0..segment_count)
        .into_iter()
        .map(|v| v * segment_size)
        .collect();
    let last = (max + 1) % segment_size;

    SegmentedRandomNumbers {
        current_segment: if segment_count == 0 {
            0
        } else {
            rng.usize(0..segment_count)
        },
        count,
        rng,
        segments,
        segment_size,
        current_index: 0,
        last_segments_size: if last == 0 { None } else { Some(last) },
        combinations_counter,
    }
}

/// Generates random numbers in a specified range. The returned struct implements the `Iterator` trait.
/// The random numbers are generated using the `fastrand` crate.
///
/// # Parameters
/// - `max`: The maximum value for the random numbers.
///
/// # Returns
/// An iterator that generates random numbers in the range `[0, max)`.
///
/// # Example
///
/// ```rust
/// let max = 10;
/// let mut rng = random_numbers(max);
///
/// for _ in 0..10 {
///    println!("{}", rng.next().unwrap());
/// }
/// ```
pub fn random_numbers(max: usize) -> impl Iterator<Item = usize> {
    struct UniqueRandomNumbers {
        rng: fastrand::Rng,
        max: usize,
    }

    impl Iterator for UniqueRandomNumbers {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let i = self.rng.usize(0..self.max);
            Some(i)
        }
    }

    let rng = fastrand::Rng::default();
    UniqueRandomNumbers { rng, max }
}

/// Generates all possible combinations of elements without considering the order.
///
/// This function generates all possible combinations of elements from a given space
/// without considering the order of elements. It uses a recursive helper function
/// `combrep` to generate the combinations.
///
/// # Type Parameters
///
/// - `LEN`: The length of each combination.
///
/// # Parameters
///
/// - `space`: The size of the space from which elements are drawn.
///
/// # Returns
///
/// A vector of arrays, where each array represents a combination of elements.
///
/// # Example
///
/// ```rust
/// let combinations = generate_no_order_combinations::<3>(5);
/// assert_eq!(combinations, vec![
///     [0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [0, 0, 4],
///     [0, 1, 1], [0, 1, 2], [0, 1, 3], [0, 1, 4],
///     [0, 2, 2], [0, 2, 3], [0, 2, 4],
///     [0, 3, 3], [0, 3, 4],
///     [0, 4, 4],
///     [1, 1, 1], [1, 1, 2], [1, 1, 3], [1, 1, 4],
///     [1, 2, 2], [1, 2, 3], [1, 2, 4],
///     [1, 3, 3], [1, 3, 4],
///     [1, 4, 4],
///     [2, 2, 2], [2, 2, 3], [2, 2, 4],
///     [2, 3, 3], [2, 3, 4],
///     [2, 4, 4],
///     [3, 3, 3], [3, 3, 4],
///     [3, 4, 4],
///     [4, 4, 4],
/// ]);
/// ```
///
/// # Panics
///
/// This function does not panic as long as the provided `space` is a valid usize.
pub fn generate_no_order_combinations<const LEN: usize>(space: usize) -> Vec<[usize; LEN]> {
    fn combrep<const LEN: usize>(
        n: usize,
        pos: usize,
        start: usize,
        l: [usize; LEN],
        result: &mut Vec<[usize; LEN]>,
    ) {
        if pos == LEN {
            result.push(l);
        } else {
            for i in start..n {
                let mut new_l = l;

                new_l[pos] = i;
                combrep(n, pos + 1, i, new_l, result);
            }
        }
    }

    let mut result = vec![];
    combrep(space, 0, 0, [0; LEN], &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashSet, hash::Hash};
    #[test]
    fn generate_index_works() {
        assert_eq!(map_to_index_space(&[2, 1], 1), [1, 0]);
        assert_eq!(map_to_index_space(&[2, 1], 0), [0, 0]);

        assert_eq!(map_to_index_space(&[2, 2], 0), [0, 0]);
        assert_eq!(map_to_index_space(&[2, 2], 1), [0, 1]);
        assert_eq!(map_to_index_space(&[2, 2], 2), [1, 0]);
        assert_eq!(map_to_index_space(&[2, 2], 3), [1, 1]);
    }

    #[test]
    fn index_to_array_works() {
        #[derive(Debug, Copy, Clone)]
        struct TestStruct {
            value: i32,
        }
        impl AsRef<TestStruct> for TestStruct {
            fn as_ref(&self) -> &TestStruct {
                &self
            }
        }
        let arrays: [&[TestStruct]; 3] = [
            &[
                TestStruct { value: 10 },
                TestStruct { value: 20 },
                TestStruct { value: 30 },
            ],
            &[
                TestStruct { value: 11 },
                TestStruct { value: 21 },
                TestStruct { value: 31 },
                TestStruct { value: 31 },
                TestStruct { value: 31 },
            ],
            &[
                TestStruct { value: 12 },
                TestStruct { value: 22 },
                TestStruct { value: 32 },
                TestStruct { value: 31 },
            ],
        ];
        let result: [&TestStruct; 3] = unsafe { select_from_arrays(&[0, 2, 1], &arrays) };
        assert_eq!(result[0].value, arrays[0][0].value,);
        assert_eq!(result[1].value, arrays[1][2].value,);
        assert_eq!(result[2].value, arrays[2][1].value,);
    }

    #[test]
    fn unique_random_numbers_works() {
        fn unordered_lists_eq<T>(a: &[T], b: &[T]) -> bool
        where
            T: Eq + Hash,
        {
            let a: HashSet<_> = a.iter().collect();
            let b: HashSet<_> = b.iter().collect();

            a == b
        }

        let counter = Arc::new(AtomicUsize::new(0));
        let test_cases = vec![
            (
                segmented_random_numbers(0, 2, counter.clone(), None).collect::<Vec<usize>>(),
                vec![0],
            ),
            (
                segmented_random_numbers(1, 2, counter.clone(), None).collect::<Vec<usize>>(),
                vec![0, 1],
            ),
            (
                segmented_random_numbers(5, 2, counter.clone(), None).collect::<Vec<usize>>(),
                vec![0, 1, 2, 3, 4, 5],
            ),
            (
                segmented_random_numbers(5, 5, counter.clone(), None).collect::<Vec<usize>>(),
                vec![0, 1, 2, 3, 4, 5],
            ),
            (
                segmented_random_numbers(7, 3, counter.clone(), None).collect::<Vec<usize>>(),
                vec![0, 1, 2, 3, 4, 5, 6, 7],
            ),
        ];
        for (index, test_case) in test_cases.iter().enumerate() {
            if !unordered_lists_eq(&test_case.0, &test_case.1) {
                panic!(
                    "wrong index({}): want {:?} but got {:?}",
                    index, test_case.1, test_case.0
                )
            }
        }
    }
    #[test]
    fn no_order_index_works() {
        assert_eq!(
            generate_no_order_combinations(5),
            [
                [0, 0],
                [0, 1],
                [0, 2],
                [0, 3],
                [0, 4],
                [1, 1],
                [1, 2],
                [1, 3],
                [1, 4],
                [2, 2],
                [2, 3],
                [2, 4],
                [3, 3],
                [3, 4],
                [4, 4],
            ]
        );
        assert_eq!(
            generate_no_order_combinations(3),
            [[0, 0], [0, 1], [0, 2], [1, 1], [1, 2], [2, 2]]
        );
        assert_eq!(
            generate_no_order_combinations(3),
            [
                [0, 0, 0],
                [0, 0, 1],
                [0, 0, 2],
                [0, 1, 1],
                [0, 1, 2],
                [0, 2, 2],
                [1, 1, 1],
                [1, 1, 2],
                [1, 2, 2],
                [2, 2, 2],
            ]
        );
        assert_eq!(
            generate_no_order_combinations(2),
            [[0, 0, 0], [0, 0, 1], [0, 1, 1], [1, 1, 1]]
        );
        assert_eq!(generate_no_order_combinations(1), [[0, 0]]);
        assert_eq!(generate_no_order_combinations(1), [[0, 0, 0]]);
    }
}

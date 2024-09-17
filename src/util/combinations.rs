use std::{
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use rayon::prelude::{ParallelBridge, ParallelIterator};

pub fn generate_full_combinations_with_random<T, TR, F, const LEN: usize>(
    segment_size: usize,
    count: Arc<AtomicUsize>,
    arrays: &[&[TR]; LEN],
    func: F,
) where
    TR: Sync + AsRef<T>,
    F: Fn([&T; LEN]) + Sync,
{
    let max_indexes: [usize; LEN] = arrays.map(|f| f.len());
    let total_combinations = max_indexes.iter().product::<usize>();

    segmented_random_numbers(total_combinations, segment_size, count)
        .par_bridge()
        .for_each(|i| {
            let index_combinations = map_to_index_space(&max_indexes, i);
            func(unsafe { select_from_arrays(&index_combinations, arrays) });
        })
}

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
) -> impl Iterator<Item = usize> {
    struct SegmentedRandomNumbers {
        rng: fastrand::Rng,
        segments: Vec<usize>,
        segment_size: usize,
        current_segment: usize,
        current_index: usize,
        count: Arc<AtomicUsize>,
        last_segments_size: Option<usize>,
    }

    impl Iterator for SegmentedRandomNumbers {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.count.fetch_add(1, Ordering::AcqRel);

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
        fn select_in_segment(&mut self) -> usize {
            let result = self.segments[self.current_segment] + self.current_index;
            self.current_index += 1;
            result
        }
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
    }
}
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
                segmented_random_numbers(0, 2, counter.clone()).collect::<Vec<usize>>(),
                vec![0],
            ),
            (
                segmented_random_numbers(1, 2, counter.clone()).collect::<Vec<usize>>(),
                vec![0, 1],
            ),
            (
                segmented_random_numbers(5, 2, counter.clone()).collect::<Vec<usize>>(),
                vec![0, 1, 2, 3, 4, 5],
            ),
            (
                segmented_random_numbers(5, 5, counter.clone()).collect::<Vec<usize>>(),
                vec![0, 1, 2, 3, 4, 5],
            ),
            (
                segmented_random_numbers(7, 3, counter.clone()).collect::<Vec<usize>>(),
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

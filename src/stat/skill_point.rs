use std::ops::{BitAnd, BitOr};
use std::{fmt, simd::i16x8};

use std::simd::cmp::{SimdOrd, SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdInt;

use crate::*;

#[derive(Debug, Default, serde::Deserialize, Clone)]
pub struct SkillPoints {
    pub assign: Point,
    pub original: Point,
}
impl SkillPoints {
    #[allow(dead_code)]
    pub fn full_put_calculate<'a, const LEN: usize>(
        items: &'a [&'a Apparel; LEN],
    ) -> (SkillPoints, [&'a Apparel; LEN]) {
        let zero = i16x8::splat(0);
        let items_req = Point::from(items.iter().fold(zero, |acc, x| acc.simd_max(x.req.inner)));

        let mut best_permutation: [&Apparel; LEN] = *items;
        let mut best_assign = Point::splat(i16::MAX);
        let mut best_original = Point::default();

        let mut permutation: [&Apparel; LEN] = *items;
        loop {
            let mut assign = Point::default();
            let mut original = Point::default();

            for item in permutation {
                let req_gap = item
                    .req
                    .inner
                    .simd_ne(zero) // ignore zero req point
                    .select((item.req.inner - original.inner).simd_max(zero), zero);
                assign.inner += req_gap;
                original.inner += req_gap;
                original.inner += item.add.inner;
            }

            // add has negative, fill it
            let negative_add_gap = items_req
                .inner
                .simd_ne(zero) // ignore zero req point
                .select((items_req.inner - original.inner).simd_max(zero), zero);
            assign.inner += negative_add_gap;
            original.inner += negative_add_gap;

            if assign.sum() < best_assign.sum() {
                best_permutation = permutation;
                best_assign = assign;
                best_original = original;
            }
            if !next_permutation_ptr(&mut permutation) {
                break;
            }
        }

        (
            SkillPoints {
                assign: best_assign,
                original: best_original,
            },
            best_permutation,
        )
    }
    #[allow(dead_code)]
    pub fn prune_put_calculate<'a, const LEN: usize>(
        items: &'a [&'a Apparel; LEN],
    ) -> (SkillPoints, [&'a Apparel; LEN]) {
        let zero = i16x8::splat(0);
        let mut best_depth_assign: [i16; LEN] = [i16::MAX; LEN];

        let prune_fn = |mut context: (SkillPoints, Point),
                        depth: usize,
                        item: &'a Apparel|
         -> Option<(SkillPoints, Point)> {
            // fill req gap
            let req_gap = item
                .req
                .inner
                .simd_ne(zero) // ignore zero req point
                .select(
                    (item.req.inner - context.0.original.inner).simd_max(zero),
                    zero,
                );
            context.0.assign.inner += req_gap;
            context.0.original.inner += req_gap;

            // add point
            context.0.original.inner += item.add.inner;

            // fill negative add gap
            let tolerance = item.req.inner.simd_ne(zero).select(item.add.inner, zero);
            context.1.inner = context.1.inner.simd_max(item.req.inner + tolerance);
            let negative_add_gap = context
                .1
                .inner
                .simd_ne(zero) // ignore zero req point
                .select(
                    (context.1.inner - context.0.original.inner).simd_max(zero),
                    zero,
                );
            context.0.assign.inner += negative_add_gap;
            context.0.original.inner += negative_add_gap;

            let sum = context.0.assign.sum();
            if sum <= best_depth_assign[depth] {
                best_depth_assign[depth] = sum;
                return Some(context);
            } else {
                return None;
            }
        };
        let best =
            bfs_permutation_with_prune(items, (SkillPoints::default(), Point::default()), prune_fn)
                .into_iter()
                .min_by(|a, b| a.1 .0.assign.sum().cmp(&b.1 .0.assign.sum()))
                .unwrap();
        (best.1 .0, best.0)
    }
    pub fn scc_put_calculate<'a, const LEN: usize>(
        items: &'a [&'a Apparel; LEN],
    ) -> (SkillPoints, [&'a Apparel; LEN]) {
        let mut depend_relation = [[false; LEN]; LEN];
        for i in 0..LEN {
            for j in i..LEN {
                depend_relation[i][j] = depend_check(items[i], items[j]);
                depend_relation[j][i] = depend_check(items[j], items[i]);
            }
        }

        // a depends on b and b depends on a -> a and b require permutation
        // d depends on c -> place c first, b second
        let depend_group = tarjans_scc(&depend_relation);

        let zero = i16x8::splat(0);
        let compute = |mut context: (SkillPoints, Point, [usize; LEN], usize),
                       index: usize|
         -> (SkillPoints, Point, [usize; LEN], usize) {
            let item = items[index];

            // permutation
            context.2[context.3] = index;
            context.3 += 1;

            // fill req gap
            let req_gap = item
                .req
                .inner
                .simd_ne(zero) // ignore zero req point
                .select(
                    (item.req.inner - context.0.original.inner).simd_max(zero),
                    zero,
                );
            context.0.assign.inner += req_gap;
            context.0.original.inner += req_gap;

            // add point
            context.0.original.inner += item.add.inner;

            // fill negative add gap
            let tolerance = item.req.inner.simd_ne(zero).select(item.add.inner, zero);
            context.1.inner = context.1.inner.simd_max(item.req.inner + tolerance);
            let negative_add_gap = context
                .1
                .inner
                .simd_ne(zero) // ignore zero req point
                .select(
                    (context.1.inner - context.0.original.inner).simd_max(zero),
                    zero,
                );
            context.0.assign.inner += negative_add_gap;
            context.0.original.inner += negative_add_gap;

            context
        };
        let best = permutation_2d_usize(
            &depend_group,
            // (skill point result, min point request, permutation array, permutation array index)
            (SkillPoints::default(), Point::default(), [0; LEN], 0),
            compute,
        )
        .into_iter()
        .min_by(|a, b| a.0.assign.sum().cmp(&b.0.assign.sum()))
        .unwrap();

        (best.0, best.2.map(|i| items[i]))
    }
    pub fn fast_gap<const LEN: usize>(items: &[&Apparel; LEN]) -> Point {
        let mut req = Point::default();
        let mut add = Point::default();
        for item in items {
            add += &item.add;

            req.inner = req.inner.simd_max(item.req.inner);
        }
        let gap = add - req;
        gap
    }
    pub fn add_weapon(&mut self, weapon: &Weapon) -> &Self {
        let zero = i16x8::splat(0);

        let mask = self.original.inner.simd_lt(weapon.req.inner);
        let gap = mask.select(weapon.req.inner - self.original.inner, zero);

        self.assign.inner += gap;
        self.original.inner += gap;
        self.original.inner += weapon.add.inner;

        self
    }
    pub fn check(&self, available_point: i16) -> bool {
        // max assign check
        let hundred = i16x8::splat(100);
        if self.assign.inner.simd_gt(hundred).any() {
            return false;
        }

        // available point check
        available_point - self.assign.inner.reduce_sum() >= 0
    }
    pub fn assign(&mut self, req: &Point) -> &Self {
        let zero = i16x8::splat(0);

        let mask = self.original.inner.simd_lt(req.inner);
        let gap = mask.select(req.inner - self.original.inner, zero);

        self.assign.inner += gap;
        self.original.inner += gap;
        self
    }
}
impl PartialEq for SkillPoints {
    fn eq(&self, other: &Self) -> bool {
        self.assign == other.assign && self.original == other.original
    }
}
impl std::fmt::Display for SkillPoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "assign:\t\t{}\noriginal:\t{}",
            self.assign, self.original
        )
    }
}

fn depend_check(b: &Apparel, a: &Apparel) -> bool {
    let zero = i16x8::splat(0);
    let a_req_lt_b_req = a.req.inner.simd_lt(b.req.inner);
    let b_add_gt_0 = b.add.inner.simd_lt(zero);
    let a_add_lt_0 = a.add.inner.simd_gt(zero);
    let b_depends_on_a = a_add_lt_0.bitand(a_req_lt_b_req.bitor(b_add_gt_0));
    b_depends_on_a.any()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn full_put_calculate_works() {
        let apparels = gen_test_apparels();
        for v in apparels {
            let apparels: [&Apparel; 8] = v.apparels.iter().collect::<Vec<_>>().try_into().unwrap();
            let (req, _) = SkillPoints::full_put_calculate(&apparels);
            assert_eq!(req, v.skill_point)
        }
    }
    #[test]
    #[ignore]
    fn prune_put_calculate_works() {
        let apparels = gen_test_apparels();
        for v in apparels {
            let apparels: [&Apparel; 8] = v.apparels.iter().collect::<Vec<_>>().try_into().unwrap();
            let (req, _) = SkillPoints::prune_put_calculate(&apparels);
            assert_eq!(req, v.skill_point)
        }
    }
    #[test]
    fn scc_put_calculate_works() {
        let apparels = gen_test_apparels();
        for v in apparels {
            let apparels: [&Apparel; 8] = v.apparels.iter().collect::<Vec<_>>().try_into().unwrap();
            let (req, _) = SkillPoints::scc_put_calculate(&apparels);
            assert_eq!(req, v.skill_point)
        }
    }
    #[test]
    fn fast_gap_works() {
        let apparels = gen_test_apparels();
        for v in apparels {
            let apparels: [&Apparel; 8] = v.apparels.iter().collect::<Vec<_>>().try_into().unwrap();
            assert_eq!(SkillPoints::fast_gap(&apparels), v.skill_point_gap)
        }
    }

    #[test]
    fn check_works() {
        assert!(SkillPoints {
            assign: Point::new(0, 0, 0, 0, 0),
            original: Point::new(100, 0, 0, 0, 0),
        }
        .check(0));
        assert!(!SkillPoints {
            assign: Point::new(101, 0, 0, 0, 0),
            original: Point::new(0, 0, 0, 0, 0),
        }
        .check(200));
        assert!(!SkillPoints {
            assign: Point::new(101, 100, 0, 0, 0),
            original: Point::new(0, 0, 0, 0, 0),
        }
        .check(200));
    }
    #[test]
    fn assign_works() {
        assert_eq!(
            SkillPoints {
                assign: Point::new(100, 0, 0, 0, 0),
                original: Point::new(100, 0, 100, 0, 0),
            }
            .assign(&Point::new(0, 5, 5, 0, 0)),
            &SkillPoints {
                assign: Point::new(100, 5, 0, 0, 0),
                original: Point::new(100, 5, 100, 0, 0),
            }
        );
    }
    #[test]
    fn with_weapon_works() {
        let weapon = Weapon {
            req: Point::new(10, 5, 0, 5, 0),
            add: Point::new(0, 0, 5, 5, 0),
            ..Default::default()
        };

        let mut no_weapon = SkillPoints {
            assign: Point::new(0, 10, 0, 0, 0),
            original: Point::new(0, 10, 0, 0, 0),
        };

        assert_eq!(
            no_weapon.add_weapon(&weapon),
            &SkillPoints {
                assign: Point::new(10, 10, 0, 5, 0),
                original: Point::new(10, 10, 5, 10, 0),
            }
        )
    }
}

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
    pub fn full_put_calculate<'a, const LEN: usize>(
        items: &'a [&'a Apparel; LEN],
    ) -> (SkillPoints, [&'a Apparel; LEN]) {
        // TODO: perf
        // ## first
        // - all req is 0
        // - The attributes of req are not added to other items
        // - all req are minimal (add has negative)
        //
        // ## sort
        // 1. All req are the smallest
        // 1. Items with the same req attribute and whose req attribute add are all 0 are merged into one item
        // 1. With the same req attribute, all items with the smallest req are in front
        // 1. Which one has the highest add of req in the same attribute? Maybe the other two sum req is less than but add is greater than
        //
        // ## last
        // - all req are maximal
        // - The properties of add and other items do not have req
        // - all add is 0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_put_calculate_works() {
        let apparels = gen_test_apparels();
        for v in apparels {
            let apparels: [&Apparel; 8] = v.apparels.iter().collect::<Vec<_>>().try_into().unwrap();
            let (req, _) = SkillPoints::full_put_calculate(&apparels);
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

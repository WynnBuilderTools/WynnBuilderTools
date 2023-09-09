use std::{
    fmt,
    simd::{i16x8, SimdInt, SimdPartialOrd},
};

use crate::*;

#[derive(Debug, Default, Hash, Clone)]
pub struct SkillPoints {
    pub assign: Point,
    pub original: Point,
}
impl SkillPoints {
    pub fn fast_put_calculate<'a, const LEN: usize>(
        items: &'a [&Apparel; LEN],
    ) -> (SkillPoints, [&'a Apparel; LEN]) {
        let zero = i16x8::splat(0);
        let mut put_perm: [&Apparel; LEN] = items.clone();
        let mut best_put_perm: [&Apparel; LEN] = put_perm;
        let mut best_assign: Point = Point::new(i16::MAX, i16::MAX, i16::MAX, i16::MAX, i16::MAX);
        let mut best_original: Point = Default::default();

        for i in 0..5 {
            let mut assign: Point = Default::default();
            let mut original: Point = Default::default();

            put_perm.sort_unstable_by_key(|v| v.req.inner[i]);
            for v in put_perm {
                let mask = original.inner.simd_lt(v.req.inner);
                let gap = mask.select(v.req.inner - original.inner, zero);
                assign.inner += gap;
                original.inner += gap;
                original.inner += v.add.inner;
            }

            if assign.all() < best_assign.all() {
                best_assign = assign;
                best_original = original;
                best_put_perm = put_perm;
            }
        }

        (
            SkillPoints {
                assign: best_assign,
                original: best_original,
            },
            best_put_perm,
        )
    }
    pub fn full_put_calculate<'a, const LEN: usize>(
        items: &'a [&'a Apparel; LEN],
    ) -> (SkillPoints, [&'a Apparel; LEN]) {
        // TODO: perf
        // ## first
        // - req is 0
        // - The attributes of req are not added to other items
        // - all reqs are minimal (add is not negative?)
        //
        // ## sort
        // 1. All reqs are the smallest
        // 1. Items with the same req attribute and whose req attribute add are all 0 are merged into one item
        // 1. With the same req attribute, all items with the smallest req are in front
        // 1. Which one has the highest add of req in the same attribute? Maybe the other two sum req is less than but add is greater than
        //
        // ## last
        // - all reqs are maximal
        // - The properties of add and other items do not have req
        // - add is 0
        let zero = i16x8::splat(0);

        let mut best_perm: [&Apparel; LEN] = items.clone();
        let mut best_assign: Point = Point::new(i16::MAX, i16::MAX, i16::MAX, i16::MAX, i16::MAX);
        let mut best_original: Point = Default::default();

        let mut put_perm: [&Apparel; LEN] = items.clone();
        loop {
            let mut assign: Point = Default::default();
            let mut original: Point = Default::default();
            for v in put_perm {
                let mask = original.inner.simd_lt(v.req.inner);
                let gap = mask.select(v.req.inner - original.inner, zero);
                assign.inner += gap;
                original.inner += gap;
                original.inner += v.add.inner;
            }
            if assign.all() < best_assign.all() {
                best_perm = put_perm;
                best_assign = assign;
                best_original = original;
            }
            if !next_permutation_ptr(&mut put_perm) {
                break;
            }
        }

        (
            SkillPoints {
                assign: best_assign,
                original: best_original,
            },
            best_perm,
        )
    }
    pub fn fast_gap<const LEN: usize>(items: &[&Apparel; LEN]) -> i16 {
        let mut req: Point = Default::default();
        let mut add: Point = Default::default();
        for item in items {
            add += &item.add;

            let mask = req.inner.simd_lt(item.req.inner);
            req.inner = mask.select(item.req.inner, req.inner)
        }
        let gap = add - req;
        gap.all()
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
    fn fast_put_calculate_works() {
        let apparels = gen_test_apparels();
        let apparels: [&Apparel; 8] = apparels
            .iter()
            .map(|s| s)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let (req, _) = SkillPoints::fast_put_calculate(&apparels);
        assert_eq!(
            req,
            SkillPoints {
                assign: Point::new(30, 50, 49, 30, 45),
                original: Point::new(30, 57, 65, 30, 45),
            }
        )
    }
    #[test]
    fn full_put_calculate_works() {
        let apparels = gen_test_apparels();
        let apparels: [&Apparel; 8] = apparels
            .iter()
            .map(|s| s)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let (req, _) = SkillPoints::full_put_calculate(&apparels);
        assert_eq!(
            req,
            SkillPoints {
                assign: Point::new(30, 50, 49, 30, 45),
                original: Point::new(30, 57, 65, 30, 45),
            }
        )
    }
    #[test]
    fn fast_gap_works() {
        let apparels = gen_test_apparels();
        let apparels: [&Apparel; 8] = apparels
            .iter()
            .map(|s| s)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        assert_eq!(SkillPoints::fast_gap(&apparels), -192)
    }

    #[test]
    fn check_works() {
        assert_eq!(
            SkillPoints {
                assign: Point::new(0, 0, 0, 0, 0),
                original: Point::new(100, 0, 0, 0, 0),
            }
            .check(0),
            true
        );
        assert_eq!(
            SkillPoints {
                assign: Point::new(101, 0, 0, 0, 0),
                original: Point::new(0, 0, 0, 0, 0),
            }
            .check(200),
            false
        );
        assert_eq!(
            SkillPoints {
                assign: Point::new(101, 100, 0, 0, 0),
                original: Point::new(0, 0, 0, 0, 0),
            }
            .check(200),
            false
        );
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
        let mut weapon = Weapon::default();
        weapon.req = Point::new(10, 5, 0, 5, 0);
        weapon.add = Point::new(0, 0, 5, 5, 0);

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

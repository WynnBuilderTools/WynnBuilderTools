use crate::items::*;

use super::skill_points_to_percentage;

pub struct Statistics {
    /// "nConvBase:4.Ice Snake Damage"
    /// "wConvBase:4.Ice Snake Damage"
    pub ability_dam_convert: DamagesConvert,
    /// "nConvBase"
    /// "wConvBase"
    pub dam_convert: DamagesConvert,
    /// str dex int def agi
    /// (e t w f a)
    pub skill_point: Point,
    /// sdPct
    pub sd_pct: f64,
    /// nSdPct
    /// wSdPct
    pub sd_pct_s: DamagesConvert,
    pub dam_pct: f64,
    pub dam_pct_s: DamagesConvert,
    pub r_sd_pct: f64,
    pub r_dam_pct: f64,
    /// rSdRaw
    pub r_sd_raw: i32,
    /// rDamRaw
    pub r_dam_raw: i32,
    pub sd_raw: i32,
    pub sd_raw_s: [i32; 6],
    /// damRaw
    pub dam_raw: i32,
    pub dam_raw_s: [i32; 6],
    /// critDamPct
    pub crit_dam_pct: f64,
    /// tDamAddMin tDamAddMax
    /// wDamAddMin wDamAddMax
    pub dam_add: Damages,
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/HEAD/js/damage_calc.js#L31
pub fn damage_calculate(
    stats: &Statistics,
    weapon: &Weapon,
    dam_convert: &DamagesConvert,
) -> (Damages, Damages) {
    // 1. Get weapon damage (with powders).

    // 2. Conversions.
    // 2.0: First, modify conversions.

    let mut conversions = dam_convert.clone();

    conversions += &stats.ability_dam_convert;
    conversions += &stats.dam_convert;

    // 2.1. First, apply neutral conversion (scale weapon damage). Keep track of total weapon damage here.

    let mut damages: Damages = weapon.damage.clone();
    let total_damages = damages.total();
    damages *= conversions.neutral();

    // 2.2. Next, apply elemental conversions using damage computed in step 1.1.
    conversions = conversions.only_positive();
    damages += &(&Damages::splat(&total_damages).only_rainbow() * &conversions);

    // 3. Apply attack speed multiplier. Ignored for melee single hit
    damages *= weapon.atk_spd.speed_mult();

    // 4. Add additive damage. TODO: Is there separate additive damage?
    let mut present = weapon.damage_present.clone();
    if conversions.neutral() == 0.0 {
        present = Mask::splat(false);
    }
    present = present | conversions.gt(0.0);

    damages += &present.select(&stats.dam_add);

    let each_weight = &damages / &damages.total();
    let rainbow_each_weight = &damages.only_rainbow() / &damages.only_rainbow().total();

    // 5. ID bonus.

    // 5.1: %boost application
    let skill_point_damage_convert: DamagesConvert =
        DamagesConvert::from_slice([0.0, 1.0, 1.0, 1.0, 0.867, 0.951]);
    let skill_boost = &DamagesConvert::from_slice([
        0.0,
        skill_points_to_percentage(stats.skill_point.e()),
        skill_points_to_percentage(stats.skill_point.t()),
        skill_points_to_percentage(stats.skill_point.w()),
        skill_points_to_percentage(stats.skill_point.f()),
        skill_points_to_percentage(stats.skill_point.a()),
    ]) * &skill_point_damage_convert;

    let mut damage_pct_s = DamagesConvert::splat(1.0);
    damage_pct_s += &skill_boost;
    damage_pct_s += stats.sd_pct + stats.dam_pct;
    damage_pct_s += &stats.sd_pct_s;
    damage_pct_s += &stats.dam_pct_s;
    damage_pct_s += &DamagesConvert::splat(stats.r_sd_pct + stats.r_dam_pct).only_rainbow();

    damages *= &damage_pct_s;

    // 5.2: Raw application.

    let mut raws: Damages = Damages::from(
        &present.select(
            &DamagesConvert::from_slice([
                stats.sd_raw_s[0] as f64 + stats.dam_raw_s[0] as f64,
                stats.sd_raw_s[1] as f64 + stats.dam_raw_s[1] as f64,
                stats.sd_raw_s[2] as f64 + stats.dam_raw_s[2] as f64,
                stats.sd_raw_s[3] as f64 + stats.dam_raw_s[3] as f64,
                stats.sd_raw_s[4] as f64 + stats.dam_raw_s[4] as f64,
                stats.sd_raw_s[5] as f64 + stats.dam_raw_s[5] as f64,
            ])
            .only_rainbow(),
        ),
    );
    let dam_raw = stats.sd_raw + stats.dam_raw;
    raws += &(&each_weight * dam_raw as f64);
    let rainbow_raw = stats.r_sd_raw + stats.r_dam_raw;
    raws += &(&rainbow_each_weight * rainbow_raw as f64);
    raws *= conversions.total();

    damages += &raws;

    // 6. Strength boosters
    let str_boost = 1.0 + skill_boost.e();
    // let mult_map = stats.dam_mult;
    // let damage_mult = 1;
    // for (const [k, v] of mult_map.entries()) {
    //     if (k.includes(':')) {
    //         // TODO: fragile... checking for specific part multipliers.
    //         const spell_match = k.split(':')[1];
    //         if (spell_match !== part_filter) {
    //             continue;
    //         }
    //     }
    //     damage_mult *= (1 + v/100);
    // }

    let crit_mult = 1.0 + stats.crit_dam_pct;

    let normal_damage = &damages * str_boost;
    let crit_damage = &damages * (str_boost + crit_mult);

    (normal_damage, crit_damage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_damage_works() {
        // test case use: https://hppeng-wynn.github.io/builder/?v=4#8_0Au0K70r50Qr0OK0K20K40OH0Qf0P0e2I1Q0e1g00010039I1004fI0z0z0+0+0+0+0---hOsKbv3 (Remix, original by RawFish)
        let mut weapon = Weapon::default();
        weapon.damage = Damages::from_slice([
            Default::default(),
            Default::default(),
            Default::default(),
            Range {
                min: 257.0,
                max: 363.0,
            },
            Default::default(),
            Default::default(),
        ]);
        weapon.damage_present = Mask::from_slice([false, false, false, true, false, false]);
        weapon.atk_spd = AtkSpd::Normal;

        let (normal_damage, crit_damage) = damage_calculate(
            &Statistics {
                ability_dam_convert: Default::default(),
                dam_convert: Default::default(),
                skill_point: Point::new(25, 40, 146, 90, 40),
                sd_pct: 0.16,
                sd_pct_s: Default::default(),
                dam_pct: 0.0,
                dam_pct_s: DamagesConvert::from_slice([0.0, 0.0, 0.23, 0.38, 0.15, 0.28]),
                r_sd_pct: 0.0,
                r_dam_pct: 0.0,
                r_sd_raw: 0,
                r_dam_raw: 0,
                sd_raw: 343,
                sd_raw_s: Default::default(),
                dam_raw: 5,
                dam_raw_s: Default::default(),
                crit_dam_pct: 0.15,
                dam_add: Damages::from_slice([
                    Default::default(),
                    Default::default(),
                    Range { min: 1.0, max: 8.0 },
                    Range { min: 2.0, max: 4.0 },
                    Range { min: 3.0, max: 5.0 },
                    Range { min: 3.0, max: 4.0 },
                ]),
            },
            &weapon,
            &DamagesConvert::from_slice([1.1, 0.3, 0.0, 0.0, 0.0, 0.0]),
        );
        assert_eq!(
            normal_damage,
            Damages::from_slice([
                Default::default(),
                Range {
                    min: 394.25,
                    max: 504.23
                },
                Default::default(),
                Range {
                    min: 2129.63,
                    max: 2818.40
                },
                Default::default(),
                Default::default(),
            ])
        );
        assert_eq!(
            crit_damage,
            Damages::from_slice([
                Default::default(),
                Range {
                    min: 765.21,
                    max: 978.68
                },
                Default::default(),
                Range {
                    min: 4133.49,
                    max: 5470.33
                },
                Default::default(),
                Default::default(),
            ])
        );
    }
}

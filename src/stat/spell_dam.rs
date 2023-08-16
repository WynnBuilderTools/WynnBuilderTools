use crate::*;

struct Statistics {
    // "nConvBase:4.Ice Snake Damage"
    // "wConvBase:4.Ice Snake Damage"
    skill_dam_convert: [f64; 6],
    // "nConvBase"
    // "wConvBase"
    dam_convert: [f64; 6],
    // str dex int def agi
    skill_point: [i32; 6],
    // sdPct
    sd_pct: f64,
    // nSdPct
    // wSdPct
    sd_pct_s: [f64; 6],
    dam_pct: f64,
    dam_pct_s: [f64; 6],
    r_sd_pct: f64,
    r_dam_pct: f64,
    // rSdRaw
    r_sd_raw: i32,
    // rDamRaw
    r_dam_raw: i32,
    sd_raw: i32,
    sd_raw_s: [i32; 6],
    // damRaw
    dam_raw: i32,
    dam_raw_s: [i32; 6],
    // critDamPct
    crit_dam_pct: f64,
    // tDamAddMin tDamAddMax
    // wDamAddMin wDamAddMax
    dam_add: [Range; 6],
}
struct Weapon {
    // "nDam_"
    // "eDam_"
    // "tDam_"
    // "wDam_"
    // "fDam_"
    // "aDam_"
    pub damage: [Range; 6],
    // damagePresent
    pub damage_present: [bool; 6],
    pub atk_spd: AtkSpd,
}
/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/HEAD/js/damage_calc.js#L31
fn calculate_spell_damage(
    stats: &Statistics,
    weapon: &Weapon,
    dam_convert: &[f64; 6],
) -> (Range, Range, Vec<[f64; 4]>) {
    // TODO: Roll all the loops together maybe

    // Array of neutral + ewtfa damages. Each entry is a pair (min, max).
    // 1. Get weapon damage (with powders).

    // 0: "nDam_"
    // 1: "eDam_"
    // 2: "tDam_"
    // 3: "wDam_"
    // 4: "fDam_"
    // 5: "aDam_"
    let weapon_damages = &weapon.damage;
    let mut present = weapon.damage_present;

    // Also theres prop and rainbow!!

    //0: "n"
    //1: "e"
    //2: "t"
    //3: "w"
    //4: "f"
    //5: "a"
    // const damage_elements = ['n'].concat(skp_elements); // netwfa

    // 2. Conversions.
    // 2.0: First, modify conversions.

    let mut conversions = dam_convert.clone();

    for i in 0..stats.skill_dam_convert.len() {
        conversions[i] += stats.skill_dam_convert[i];
    }
    for (i, element) in stats.dam_convert.iter().enumerate() {
        conversions[i] += element;
    }

    // 2.1. First, apply neutral conversion (scale weapon damage). Keep track of total weapon damage here.
    let mut damages = Vec::new();
    let neutral_convert = conversions[0];
    if neutral_convert == 0.0 {
        present = [false, false, false, false, false, false]
    }
    let mut weapon_min = 0.0;
    let mut weapon_max = 0.0;
    for damage in weapon_damages {
        let min = damage.min * neutral_convert;
        let max = damage.max * neutral_convert;
        damages.push(Range { min, max });
        weapon_min += damage.min;
        weapon_max += damage.max;
    }

    // 2.2. Next, apply elemental conversions using damage computed in step 1.1.
    // Also, track which elements are present. (Add onto those present in the weapon itself.)
    let mut total_convert = 0.0;
    for i in 1..conversions.len() {
        if conversions[i] > 0.0 {
            damages[i].min += conversions[i] * weapon_min;
            damages[i].max += conversions[i] * weapon_max;
            present[i] = true;
            total_convert += conversions[i]
        }
    }
    total_convert += conversions[0];

    // 3. Apply attack speed multiplier. Ignored for melee single hit
    let attack_speed_mult = weapon.atk_spd.speed_mult();
    for i in 0..damages.len() {
        damages[i].min *= attack_speed_mult;
        damages[i].max *= attack_speed_mult;
    }

    // 4. Add additive damage. TODO: Is there separate additive damage?
    for i in 0..stats.dam_add.len() {
        if present[i] {
            damages[i].min += stats.dam_add[i].min;
            damages[i].max += stats.dam_add[i].max;
        }
    }

    // 5. ID bonus.
    // let specific_boost_str = 'Md';
    // if use_spell_damage {
    //     specific_boost_str = 'Sd';
    // }

    // 5.1: %boost application
    let skillpoint_damage_mult: [f64; 6] = [1.0, 1.0, 1.0, 1.0, 0.867, 0.951];
    let mut skill_boost = Vec::new(); // no neutral skillpoint booster
    for (i, v) in stats.skill_point.iter().enumerate() {
        skill_boost.push(skill_points_to_percentage(*v) * skillpoint_damage_mult[i]);
    }
    let static_boost = stats.sd_pct + stats.dam_pct;

    // These do not count raw damage. I think. Easy enough to change
    let mut total_min = 0.0;
    let mut total_max = 0.0;
    let mut save_prop = Vec::new();
    for i in 0..6 {
        save_prop.push(damages[i].clone());
        total_min += damages[i].min;
        total_max += damages[i].max;

        // let damage_specific = damage_elements[i] + specific_boost_str + 'Pct';
        let mut damage_boost =
            1.0 + skill_boost[i] + static_boost + stats.sd_pct_s[i] + stats.dam_pct_s[i];
        if i > 0 {
            damage_boost += stats.r_sd_pct + stats.r_dam_pct;
        }
        damages[i].min *= num::Float::max(damage_boost, 0.0);
        damages[i].max *= num::Float::max(damage_boost, 0.0);
        // Collect total damage post %boost
    }

    let total_elem_min = total_min - damages[0].min;
    let total_elem_max = total_max - damages[0].max;

    // 5.2: Raw application.
    let prop_raw = stats.sd_raw + stats.dam_raw;
    let rainbow_raw = stats.r_sd_raw + stats.r_dam_raw;
    for i in 0..damages.len() {
        let save_obj = &save_prop[i];
        let damages_obj = &mut damages[i];
        // Normie raw
        let mut raw_boost = 0.0;
        if present[i] {
            raw_boost += stats.sd_raw_s[i] as f64 + stats.dam_raw_s[i] as f64;
        }
        // Next, rainraw and propRaw
        let mut min_boost = raw_boost;
        let mut max_boost = raw_boost;
        if total_max > 0.0 {
            // TODO: what about total negative all raw?
            // TODO: compute actual chance of 0 damage. For now we just copy max ratio
            if total_min == 0.0 {
                min_boost += (save_obj.max / total_max) * prop_raw as f64;
            } else {
                min_boost += (save_obj.min / total_min) * prop_raw as f64;
            }
            max_boost += (save_obj.max / total_max) * prop_raw as f64;
        }
        if i != 0 && total_elem_max > 0.0 {
            // rainraw    TODO above
            // TODO: compute actual chance of 0 damage. For now we just copy max ratio
            if total_elem_min == 0.0 {
                min_boost += (save_obj.max / total_elem_max) * rainbow_raw as f64;
            } else {
                min_boost += (save_obj.min / total_elem_min) * rainbow_raw as f64;
            }
            max_boost += (save_obj.max / total_elem_max) * rainbow_raw as f64;
        }
        damages_obj.min += min_boost * total_convert;
        damages_obj.max += max_boost * total_convert;
    }

    // 6. Strength boosters
    // str/dex, as well as any other mutually multiplicative effects
    let str_boost = 1.0 + skill_boost[1];
    let mut total_dam_norm = Range { min: 0.0, max: 0.0 };
    let mut total_dam_crit = Range { min: 0.0, max: 0.0 };
    let mut damages_results = Vec::new();
    // let mult_map = stats.dam_mult;
    let damage_mult = 1;
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

    for damage in damages {
        let res = [
            damage.min * str_boost * damage_mult as f64, // Normal min
            damage.max * str_boost * damage_mult as f64, // Normal max
            damage.min * (str_boost + crit_mult) * damage_mult as f64, // Crit min
            damage.max * (str_boost + crit_mult) * damage_mult as f64, // Crit max
        ];
        damages_results.push(res);
        total_dam_norm.min += res[0];
        total_dam_norm.max += res[1];
        total_dam_crit.min += res[2];
        total_dam_crit.max += res[3];
    }

    if total_dam_norm.min < 0.0 {
        total_dam_norm.min = 0.0;
    }
    if total_dam_norm.max < 0.0 {
        total_dam_norm.max = 0.0;
    }
    if total_dam_crit.min < 0.0 {
        total_dam_crit.min = 0.0;
    }
    if total_dam_crit.max < 0.0 {
        total_dam_crit.max = 0.0;
    }

    return (total_dam_norm, total_dam_crit, damages_results);
}

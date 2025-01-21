use std::collections::{BTreeMap, HashMap};

use crate::items::*;

/// return (common_stat, dam_raw, dam_pct, dam_add, spells)
/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/50ed4620bd0a4e3af7dd5646971c6dcd78e8b783/js/builder/atree.js#L441
pub fn atree_merge<'a>(
    active_abilities: &Vec<&ATreeNodeData>,
) -> (CommonStat, i32, Dam, Damages, Vec<Spell>) {
    let abilities_merged = merge_ability(active_abilities);

    let (common_stat, dam_raw, dam_pct, dam_add): (CommonStat, i32, Dam, Damages) =
        abilities_merged
            .values()
            .map(|v| v.join_stat())
            .fold(Default::default(), |mut acc, x| {
                acc.0 += &x.0;
                acc.1 += &x.1;
                acc.2 += &x.2;
                acc.3 += &x.3;
                acc
            });

    let (mut base_spells, merged_spell_properties): (
        HashMap<i32, Spell>,
        HashMap<i32, (i32, Vec<DamagePart>)>,
        // reverses abilities, only uses the last spell(replace_spell) as the base spell
    ) = abilities_merged.iter().rev().fold(
        (HashMap::new(), HashMap::new()),
        |(mut base_spells, mut merged_spell_properties), (_, ability)| {
            let spells = ability.join_spell();
            let spell_properties = ability.join_spell_property();

            for (base_id, spell) in spells {
                base_spells.entry(base_id).or_insert(spell);
            }
            for (base_id, (cost, mut parts)) in spell_properties {
                merged_spell_properties
                    .entry(base_id)
                    .and_modify(|(cost_modify, parts_modify)| {
                        *cost_modify += cost;
                        parts_modify.append(&mut parts);
                    })
                    .or_insert((cost, parts));
            }
            (base_spells, merged_spell_properties)
        },
    );

    for (base_id, (cost, add_parts)) in merged_spell_properties {
        if let Some(value) = base_spells.get_mut(&base_id) {
            value.cost += cost;
            for add_part in add_parts {
                if let Some(v) = value.parts.iter_mut().find(|v| v.name == add_part.name) {
                    v.dam_convert += &add_part.dam_convert;
                } else {
                    value.parts.push(add_part);
                }
            }
        } else {
            println!("not found base ability: {}", base_id);
        }
    }

    (
        common_stat,
        dam_raw,
        dam_pct,
        dam_add,
        base_spells.into_iter().map(|(_key, value)| value).collect(),
    )
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/50ed4620bd0a4e3af7dd5646971c6dcd78e8b783/js/builder/atree.js#L464
pub fn merge_ability(active_abilities: &Vec<&ATreeNodeData>) -> BTreeMap<i32, ATreeNodeData> {
    let mut abilities_merged = BTreeMap::<i32, ATreeNodeData>::new();

    // insert base ability
    active_abilities
        .iter()
        .filter(|v| v.base_abil.is_none())
        .for_each(|&v| {
            abilities_merged.insert(v.id, v.clone());
        });

    // merge not base ability
    active_abilities
        .iter()
        .filter_map(|v| v.base_abil.map(|base_abil_id| (base_abil_id, v)))
        .for_each(|(base_id, &v)| {
            if let Some(base_ability) = abilities_merged.get_mut(&base_id) {
                base_ability.effects.extend(v.effects.clone());
            } else {
                // https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/f29e47836e7469bae4eed1264ccc113c516fc73c/js/builder/atree.js#L131
                // 999 is "Melee", this type of calculation is not currently supported
                if base_id == 999 {
                    return;
                }
                // https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/f29e47836e7469bae4eed1264ccc113c516fc73c/js/builder/atree.js#L128
                // 998 is "Elemental Mastery",
                // for example, "Air Mastery": "Increase your base damage from all Air attacks"
                // it has same effect as a base ability
                if base_id == 998 {
                    abilities_merged.insert(v.id, v.clone());
                    return;
                }

                println!(
                    "Base ability not found, base_abil_id: {}, abil is {}",
                    base_id, v.display_name,
                );
            }
        });

    abilities_merged
}

// TODO merge_major_id
// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/50ed4620bd0a4e3af7dd5646971c6dcd78e8b783/js/builder/atree.js#L502

#[cfg(test)]
mod tests {
    use crate::calculate::decode_atree;
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn test_atree_merge() {
        let file = File::open("assets/atree_test_case.json")
            .expect("The file `atree.json` should exist in the folder assets.");
        let reader = BufReader::new(file);

        let abilities: AbilitiesMap = serde_json::from_reader(reader).unwrap();

        // https://hppeng-wynn.github.io/builder/?v=10#9_2SG2SH2SI2SJ2SK2SL2SM2SN0Qf00002I00001g000000z0z0+0+0+0+0-1T--hOsK5v3
        let active_abilities = decode_atree(&abilities.get(&Class::Warrior).unwrap(), "--hOsK5v3");
        let (common_stat, dam_raw, dam_pct, dam_add, spells) = atree_merge(&active_abilities);
        assert_eq!(common_stat, CommonStat::new(0, 0, 0, 0, 0, 20, 0, 0));
        assert_eq!(5, dam_raw);
        assert_eq!(
            DamagesConvert::from_slice([0.0, 0.0, 0.1, 0.15, 0.15, 0.15]),
            DamagesConvert::from(&dam_pct)
        );
        assert_eq!(
            Damages::from_slice([
                Default::default(),
                Default::default(),
                Range { min: 1.0, max: 8.0 },
                Range { min: 2.0, max: 4.0 },
                Range { min: 3.0, max: 5.0 },
                Range { min: 3.0, max: 4.0 },
            ]),
            dam_add
        );

        for v in spells {
            if v.id != 3 {
                continue;
            }
            assert_eq!(
                Spell::new(
                    "Uppercut".to_string(),
                    3,
                    45,
                    vec![
                        DamagePart::new(
                            "Uppercut".to_string(),
                            DamagesConvert::from_slice([3.2, 0.4, 0.4, 0.3, 0.0, 0.3])
                        ),
                        DamagePart::new(
                            "Fireworks".to_string(),
                            DamagesConvert::from_slice([0.8, 0.0, 0.2, 0.0, 0.0, 0.0])
                        )
                    ]
                ),
                v
            );
        }
    }
}

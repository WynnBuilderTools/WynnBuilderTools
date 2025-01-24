mod ability;
mod apparel;
mod atk_spd;
mod class;
mod common_stat;
mod dam;
mod damages;
mod point;
mod range;
mod sec_stat;
mod spell;
mod wapi_item;
mod weapon;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub use ability::*;
pub use apparel::*;
pub use atk_spd::*;
pub use class::*;
pub use common_stat::*;
pub use dam::*;
pub use damages::*;
pub use point::*;
pub use range::*;
pub use sec_stat::*;
pub use spell::*;
pub use wapi_item::*;
pub use weapon::*;

pub fn load_hppeng_id_map() -> HashMap<String, i32> {
    let path = "assets/id_map.json";
    let mut file = File::open(path).expect("fs should be able to open id_map.json file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("fs should be able to read id_map.json file");
    let id_map: HashMap<String, i32> = serde_json::from_str(&contents).unwrap();
    id_map
}

pub fn load_from_wapi<P>(path: P) -> Result<([Vec<Apparel>; 7], Vec<Weapon>), String>
where
    P: AsRef<Path>,
{
    let file_result: Result<File, std::io::Error> = File::open(path);

    if let Ok(file) = file_result {
        let reader = BufReader::new(file);

        let items: WApiItems = serde_json::from_reader(reader).unwrap();

        let mut apparels: [Vec<Apparel>; 7] = Default::default();
        let mut weapons: Vec<Weapon> = Vec::new();
        items.iter().for_each(|(_, value)| match &value.r#type {
            Some(item_type) => match item_type {
                ItemType::Material => (),
                ItemType::Tool => (),
                ItemType::Ingredient => (),
                ItemType::Charm => (),
                ItemType::Tome => (),
                ItemType::Accessory => {
                    match &value.accessory_type {
                        Some(accessory_type) => match accessory_type {
                            AccessoryType::Bracelet => {
                                apparels[5].push(Apparel::try_from(value).unwrap())
                            }
                            AccessoryType::Necklace => {
                                apparels[6].push(Apparel::try_from(value).unwrap())
                            }
                            AccessoryType::Ring => {
                                apparels[4].push(Apparel::try_from(value).unwrap())
                            }
                        },
                        None => (),
                    };
                }
                ItemType::Armour => {
                    match &value.armour_type {
                        Some(armour_type) => match armour_type {
                            ArmourType::Boots => {
                                apparels[3].push(Apparel::try_from(value).unwrap())
                            }
                            ArmourType::Chestplate => {
                                apparels[1].push(Apparel::try_from(value).unwrap())
                            }
                            ArmourType::Helmet => {
                                apparels[0].push(Apparel::try_from(value).unwrap())
                            }
                            ArmourType::Leggings => {
                                apparels[2].push(Apparel::try_from(value).unwrap())
                            }
                        },
                        None => (),
                    };
                }
                ItemType::Weapon => weapons.push(Weapon::try_from(value).unwrap()),
            },
            None => (),
        });

        Ok((apparels, weapons))
    } else {
        Err("Could not open file".to_string())
    }
}

mod apparel;
mod atk_spd;
mod class;
mod comm_stat;
mod dam;
mod damages;
mod items;
mod point;
mod range;
mod weapon;

pub use apparel::*;
pub use atk_spd::*;
pub use class::*;
pub use comm_stat::*;
pub use dam::*;
pub use damages::*;
pub use items::*;
pub use point::*;
pub use range::*;
pub use weapon::*;

use std::{fs::File, io::BufReader, path::Path};

pub fn load_from_json<P>(path: P) -> ([Vec<Apparel>; 7], Vec<Weapon>)
where
    P: AsRef<Path>,
{
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let items: Items = serde_json::from_reader(reader).unwrap();
    let mut apparels: [Vec<Apparel>; 7] = Default::default();
    let mut weapons: Vec<Weapon> = Vec::new();
    items
        .items
        .iter()
        .for_each(|value| match value.r#type.as_str() {
            "helmet" => apparels[0].push(Apparel::try_from(value).unwrap()),
            "chestplate" => apparels[1].push(Apparel::try_from(value).unwrap()),
            "leggings" => apparels[2].push(Apparel::try_from(value).unwrap()),
            "boots" => apparels[3].push(Apparel::try_from(value).unwrap()),
            "ring" => apparels[4].push(Apparel::try_from(value).unwrap()),
            "bracelet" => apparels[5].push(Apparel::try_from(value).unwrap()),
            "necklace" => apparels[6].push(Apparel::try_from(value).unwrap()),
            "relik" | "bow" | "wand" | "dagger" | "spear" => {
                weapons.push(Weapon::try_from(value).unwrap())
            }
            _ => (),
        });

    (apparels, weapons)
}

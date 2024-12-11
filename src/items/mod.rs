mod api_items;
mod apparel;
mod atk_spd;
mod class;
mod common_stat;
mod dam;
mod damages;
mod item;
mod point;
mod range;
mod sec_stat;
mod weapon;

use std::{fs::File, io::BufReader, path::Path};

pub use api_items::*;
pub use apparel::*;
pub use atk_spd::*;
pub use class::*;
pub use common_stat::*;
pub use dam::*;
pub use damages::*;
pub use item::*;
pub use point::*;
pub use range::*;
pub use sec_stat::*;
pub use weapon::*;

/// Load items from a JSON file
///
/// # Arguments
///
/// - `path` - A path to the JSON file
///
/// # Returns
///
/// A tuple containing a list of gear and weapons
///
/// # Example
///
/// ```rust
/// use std::path::Path;
/// use items::load_from_json;
///
/// let (apparels, weapons) = load_from_json(Path::new("items.json"));
/// ```
///
/// # Panics
///
/// This function will panic if the file cannot be opened or if the JSON file is invalid
pub fn load_from_json<P>(path: P) -> Result<([Vec<Apparel>; 7], Vec<Weapon>), String>
where
    P: AsRef<Path>,
{
    let file_result: Result<File, std::io::Error> = File::open(path);

    if let Ok(file) = file_result {
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

        Ok((apparels, weapons))
    } else {
        Err("Could not open file".to_string())
    }
}

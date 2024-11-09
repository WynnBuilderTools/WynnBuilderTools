use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MajorID {
    pub display_name: String,
    pub description: String,
    pub abilities: Vec<MajorIDAbility>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MajorIDAbility {
    class: String,
    base_abil: i32,
    effects: Vec<Effect>,
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs::File, io::BufReader};

    use super::*;

    #[test]
    fn derivative_works() {
        let file = File::open("assets/majid.json")
            .expect("The file `majid.json` should exist in the folder assets.");
        let reader = BufReader::new(file);

        let _: HashMap<String, MajorID> = serde_json::from_reader(reader).unwrap();
    }
}

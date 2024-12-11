use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::hash::Hash;

use url::Url;

use crate::*;

/// ## url index:
/// fragment example: #9_0Au0K70r50Qr0OK0K20K40OH0Qf160e2I1S0e1g00010039I1004fI0z0z0+0+0+0+0o1T--hOsKbv3
/// - version "9"
/// - apparels+weapon "0Au0K70r50Qr0OK0K20K40OH0Qf" [len 27]: https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/197e50863b366a32251dc77c0511d96004d754d4/js/builder/build_encode_decode.js#L153
/// - skill point "160e2I1S0e" [len 10]: https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/197e50863b366a32251dc77c0511d96004d754d4/js/builder/build_encode_decode.js#L224
/// - level "1g" [len 2]: https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/197e50863b366a32251dc77c0511d96004d754d4/js/builder/build_encode_decode.js#L221
/// - powder "00010039I1004fI" [len 5*(1+5*?)]: https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/197e50863b366a32251dc77c0511d96004d754d4/js/builder/build_encode_decode.js#L231
/// - tomes "0z0z0+0+0+0+0o1T" [len 16]: https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/197e50863b366a32251dc77c0511d96004d754d4/js/builder/build_encode_decode.js#L235
/// - ability "--hOsKbv3" [len last]: https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/197e50863b366a32251dc77c0511d96004d754d4/js/builder/build_encode_decode.js#L268
#[derive(Debug, Clone, PartialEq)]
pub struct HppengCodes {
    pub prefix: String,
    pub version: String,
    pub items: String,
    pub skill_point: String,
    pub level: String,
    pub powder: String,
    pub tomes: String,
    pub ability: String,
}
impl HppengCodes {
    pub fn split_hppeng_url(url: &str) -> Self {
        let mut url = Url::parse(url).unwrap();
        let fragment = url.fragment().unwrap().to_owned();

        url.set_fragment(None);
        let url_prefix = url.to_string();

        let version = fragment[0..1].to_string();
        let items_end = 2 + 27;
        let items = fragment[2..items_end].to_string();
        let skill_point_end = items_end + 10;
        let skill_point = fragment[items_end..skill_point_end].to_string();
        let level_end = skill_point_end + 2;
        let level = fragment[skill_point_end..level_end].to_string();

        let powder_length = calculate_powder_length(&fragment[level_end..].to_string());
        let powder_end = level_end + powder_length;
        let powder = fragment[level_end..powder_end].to_string();

        let tomes_end = powder_end + 16;
        let tomes = fragment[powder_end..tomes_end].to_string();
        let ability = fragment[tomes_end..].to_string();

        Self {
            prefix: url_prefix,
            version,
            items,
            skill_point,
            level,
            powder,
            tomes,
            ability,
        }
    }
    pub fn generate_url(
        &self,
        version: Option<&str>,
        items: Option<[i32; 9]>,
        skill_point: Option<[i32; 5]>,
        level: Option<i32>,
    ) -> String {
        let items = items.map(|value| value.map(|id| from_int_n(id, 3)).join(""));
        let skill_point = skill_point.map(|value| value.map(|point| from_int_n(point, 2)).join(""));
        let level = level.map(|value| from_int_n(value, 2));

        let mut template = String::new();
        write!(&mut template, "{}", self.prefix).unwrap();
        write!(&mut template, "#{}_", version.unwrap_or(&self.version)).unwrap();
        write!(&mut template, "{}", items.unwrap_or(self.items.clone())).unwrap();
        write!(
            &mut template,
            "{}",
            skill_point.unwrap_or(self.skill_point.clone())
        )
        .unwrap();
        write!(&mut template, "{}", level.unwrap_or(self.level.clone())).unwrap();
        write!(&mut template, "{}", self.powder).unwrap();
        write!(&mut template, "{}", self.tomes).unwrap();
        write!(&mut template, "{}", self.ability).unwrap();
        template
    }
}
/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/d952c489f021694113ef89cd0a7452c42ce0ccac/js/builder/build_encode_decode.js#L11
fn calculate_powder_length(mut powder_info: &str) -> usize {
    let mut total_length = 0;

    for _ in 0..5 {
        if powder_info.is_empty() {
            break;
        }
        let n_blocks = match powder_info.chars().next() {
            Some(c) => c,
            None => break,
        };
        total_length += 1;
        let n_blocks = to_int(&n_blocks.to_string()) as usize;

        total_length += n_blocks * 5;
        powder_info = &powder_info[1 + n_blocks * 5..];
    }

    total_length
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/197e50863b366a32251dc77c0511d96004d754d4/js/utils.js#L87
const CUSTOM_DIGITS: [char; 64] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', '+', '-',
];

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/197e50863b366a32251dc77c0511d96004d754d4/js/utils.js#L108
fn from_int_n(int32: i32, n: u64) -> String {
    let mut result = String::new();
    let mut int32 = int32;

    for _ in 0..n {
        let digit = CUSTOM_DIGITS[(int32 & 0x3f) as usize];
        result.insert(0, digit);
        int32 >>= 6;
    }

    result
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/197e50863b366a32251dc77c0511d96004d754d4/js/utils.js#L116
fn to_int(digits_str: &str) -> u64 {
    let mut result = 0u64;
    for digit in digits_str.chars() {
        if let Some(index) = CUSTOM_DIGITS.iter().position(|&ch| ch == digit) {
            result = (result << 6) + index as u64;
        } else {
            panic!("Invalid character in digits string: {}", digit);
        }
    }
    result
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/197e50863b366a32251dc77c0511d96004d754d4/js/utils.js#L202
fn to_base64_bits(digits_str: &str) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();

    for digit in digits_str.chars() {
        if let Some(index) = CUSTOM_DIGITS.iter().position(|&ch| ch == digit) {
            for j in 0..6 {
                let bit = (index >> j) & 1;
                result.push(bit == 1);
            }
        } else {
            panic!("Invalid character in digits string: {}", digit);
        }
    }

    result
}

pub trait TreeNode {
    type NodeIDType: Eq + Hash + Clone + Ord;
    fn parents(&self) -> &Vec<Self::NodeIDType>;
    fn id(&self) -> Self::NodeIDType;
}
impl TreeNode for ATreeNodeData {
    type NodeIDType = i32;

    fn parents(&self) -> &Vec<Self::NodeIDType> {
        &self.parents
    }

    fn id(&self) -> Self::NodeIDType {
        self.id
    }
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/197e50863b366a32251dc77c0511d96004d754d4/js/builder/build_encode_decode.js#L416
pub fn decode_atree<'a>(
    abilities: &'a Vec<ATreeNodeData>,
    atree_url: &'a str,
) -> Vec<&'a ATreeNodeData> {
    let mut active_status = to_base64_bits(atree_url);
    let graph = generate_graph(abilities);

    // add head
    // https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/197e50863b366a32251dc77c0511d96004d754d4/js/builder/build_encode_decode.js#L419
    active_status.insert(0, true);

    // sort tree node vector by Depth-first search
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let start_node = abilities.iter().find(|v| v.parents().len() == 0).unwrap();
    stack.push(start_node);
    active_status.reverse();
    while let Some(node) = stack.pop() {
        // the following order cannot be disrupted
        // 1
        if visited.contains(&node.id()) {
            continue;
        }
        // 2
        visited.insert(node.id());
        // 3
        if let Some(active) = active_status.pop() {
            if !active {
                continue;
            }
        }
        // 4
        result.push(node);
        if let Some(children) = graph.get(&node.id()) {
            for &child in children.iter().rev() {
                stack.push(child);
            }
        }
    }

    result
}

// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/50ed4620bd0a4e3af7dd5646971c6dcd78e8b783/js/builder/atree.js#L191
fn generate_graph<T: TreeNode>(nodes: &Vec<T>) -> HashMap<T::NodeIDType, Vec<&T>> {
    let mut graph: HashMap<T::NodeIDType, Vec<&T>> = HashMap::new();

    for node in nodes {
        for parent in node.parents() {
            graph
                .entry(parent.clone())
                .or_insert_with(Vec::new)
                .push(node);
        }
    }

    for (_, children) in &mut graph {
        children.sort_by(|a, b| a.id().cmp(&b.id()));
    }

    graph
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn test_generate_graph() {
        let children_vector: Vec<(u32, Vec<u32>)> = vec![
            (0, vec![1]),
            (1, vec![2, 3]),
            (3, vec![4]),
            (4, vec![6, 7]),
            (7, vec![10]),
            (6, vec![8]),
            (8, vec![5, 9, 11, 12]),
            (9, vec![8, 10, 12, 13, 14]),
            (10, vec![9, 14, 15]),
            (11, vec![16]),
            (15, vec![20]),
            (14, vec![12, 13, 19]),
            (12, vec![13, 14, 17]),
            (19, vec![20, 21]),
            (20, vec![19, 21]),
            (21, vec![24]),
            (24, vec![29]),
            (17, vec![16, 25]),
            (16, vec![17, 25]),
            (25, vec![26]),
            (13, vec![18]),
            (18, vec![23]),
            (23, vec![22, 27]),
            (22, vec![27]),
            (27, vec![26, 28, 30, 33, 44]),
            (26, vec![27, 32, 44]),
            (28, vec![27, 29, 30, 31]),
            (29, vec![28, 31, 35]),
            (35, vec![34, 41]),
            (34, vec![33, 35, 37, 39]),
            (33, vec![34, 37, 39, 43]),
            (39, vec![40, 43, 46, 50]),
            (40, vec![39, 41, 46, 47]),
            (41, vec![40, 47]),
            (46, vec![]),
            (47, vec![51]),
            (51, vec![53, 54]),
            (54, vec![53, 61]),
            (53, vec![61]),
            (37, vec![]),
            (31, vec![30]),
            (30, vec![31]),
            (44, vec![]),
            (32, vec![36, 38]),
            (38, vec![]),
            (36, vec![42]),
            (42, vec![43, 45, 48]),
            (43, vec![42, 45]),
            (48, vec![49, 52, 55]),
            (49, vec![48, 50, 52]),
            (50, vec![49, 57]),
            (52, vec![]),
            (57, vec![56, 59, 60, 66]),
            (56, vec![55, 57, 58, 59]),
            (55, vec![56, 58, 62]),
            (60, vec![61]),
            (61, vec![60, 67]),
            (67, vec![65, 66, 68]),
            (66, vec![64, 65, 67]),
            (68, vec![71, 72]),
            (72, vec![]),
            (64, vec![]),
            (65, vec![]),
            (62, vec![63, 70]),
            (70, vec![69, 73]),
            (73, vec![]),
            (69, vec![71]),
            (71, vec![69]),
            (63, vec![]),
            (58, vec![59]),
            (59, vec![58]),
            (45, vec![]),
            (5, vec![]),
            (2, vec![]),
        ];
        // warrior ability
        let parents_vector = vec![
            (0, vec![]),
            (1, vec![0]),
            (3, vec![1]),
            (4, vec![3]),
            (7, vec![4]),
            (6, vec![4]),
            (8, vec![6, 9]),
            (9, vec![8, 10]),
            (10, vec![7, 9]),
            (11, vec![8]),
            (15, vec![10]),
            (14, vec![10, 12, 9]),
            (12, vec![8, 14, 9]),
            (19, vec![14, 20]),
            (20, vec![15, 19]),
            (21, vec![19, 20]),
            (24, vec![21]),
            (17, vec![12, 16]),
            (16, vec![11, 17]),
            (25, vec![16, 17]),
            (13, vec![9, 12, 14]),
            (18, vec![13]),
            (23, vec![18]),
            (22, vec![23]),
            (27, vec![26, 28, 23, 22]),
            (26, vec![25, 27]),
            (28, vec![27, 29]),
            (29, vec![28, 24]),
            (35, vec![34, 29]),
            (34, vec![35, 33]),
            (33, vec![27, 34]),
            (39, vec![34, 33, 40]),
            (40, vec![39, 41]),
            (41, vec![40, 35]),
            (46, vec![39, 40]),
            (47, vec![40, 41]),
            (51, vec![47]),
            (54, vec![51]),
            (53, vec![51, 54]),
            (37, vec![34, 33]),
            (31, vec![28, 30, 29]),
            (30, vec![28, 27, 31]),
            (44, vec![26, 27]),
            (32, vec![26]),
            (38, vec![32]),
            (36, vec![32]),
            (42, vec![36, 43]),
            (43, vec![42, 33, 39]),
            (48, vec![42, 49]),
            (49, vec![50, 48]),
            (50, vec![39, 49]),
            (52, vec![48, 49]),
            (57, vec![56, 50]),
            (56, vec![55, 57]),
            (55, vec![56, 48]),
            (60, vec![57, 61]),
            (61, vec![60, 53, 54]),
            (67, vec![61, 66]),
            (66, vec![57, 67]),
            (68, vec![67]),
            (72, vec![68]),
            (64, vec![66]),
            (65, vec![66, 67]),
            (62, vec![55]),
            (70, vec![62]),
            (73, vec![70]),
            (69, vec![70, 71]),
            (71, vec![68, 69]),
            (63, vec![62]),
            (58, vec![56, 55, 59]),
            (59, vec![56, 57, 58]),
            (45, vec![42, 43]),
            (5, vec![8]),
            (2, vec![1]),
        ];

        let nodes: Vec<MyNode> = parents_vector
            .into_iter()
            .map(|(id, parents)| MyNode { id, parents })
            .collect();

        let mut expectation: HashMap<u32, Vec<MyNode>> = HashMap::new();
        children_vector.into_iter().for_each(|(id, children)| {
            expectation.insert(
                id,
                children
                    .into_iter()
                    .map(|v| MyNode {
                        id: v,
                        parents: vec![],
                    })
                    .collect(),
            );
        });
        for (key, children) in generate_graph(&nodes) {
            if let Some(v) = expectation.get(&key) {
                assert_eq!(
                    v.iter().map(|v| v.id).collect::<Vec<u32>>(),
                    children.iter().map(|v| v.id).collect::<Vec<u32>>()
                );
            } else {
                panic!("not found {}", key)
            };
        }
    }

    #[derive(Debug, Eq, PartialEq, Hash)]
    struct MyNode {
        id: u32,
        parents: Vec<u32>,
    }

    impl TreeNode for MyNode {
        type NodeIDType = u32;

        fn parents(&self) -> &Vec<Self::NodeIDType> {
            &self.parents
        }

        fn id(&self) -> Self::NodeIDType {
            self.id
        }
    }

    #[test]
    fn split_hppeng_url_works() {
        assert_eq!(
            HppengCodes {
                prefix: "https://hppeng-wynn.github.io/builder/?v=10".to_owned(),
                version: "9".to_owned(),
                items: "2SG2SH2SI2SJ2SK2SL2SM2SN0Qf".to_owned(),
                skill_point: "00002I0000".to_owned(),
                level: "1g".to_owned(),
                powder: "00000".to_owned(),
                tomes: "0z0z0+0+0+0+0-1T".to_owned(),
                ability: "--hOsK5v3".to_owned(),
            },
            HppengCodes::split_hppeng_url("https://hppeng-wynn.github.io/builder/?v=10#9_2SG2SH2SI2SJ2SK2SL2SM2SN0Qf00002I00001g000000z0z0+0+0+0+0-1T--hOsK5v3")
        )
    }
    #[test]
    fn generate_url_works() {
        let test = HppengCodes {
            prefix: "".to_owned(),
            version: "".to_owned(),
            items: "".to_owned(),
            skill_point: "".to_owned(),
            level: "".to_owned(),
            powder: "".to_owned(),
            tomes: "".to_owned(),
            ability: "".to_owned(),
        };
        assert_eq!(
            "#9_04004B04C0482SK2SL2SM2SN03E00000000001g",
            test.generate_url(
                Some("9"),
                Some([256, 267, 268, 264, 10004, 10005, 10006, 10007, 206]),
                Some([0, 0, 0, 0, 0]),
                Some(106),
            )
        );
    }
    #[test]
    fn calculate_powder_length_works() {
        assert_eq!(
            15,
            calculate_powder_length("00010039I1004fI0z0z0+0+0+0+0o1T--hOsKbv3")
        );
    }

    #[test]
    fn from_int_n_works() {
        assert_eq!("JI", from_int_n(1234, 2));
    }
    #[test]
    fn to_int_works() {
        assert_eq!(1234, to_int("JI"));
    }
    #[test]
    fn string_to_bit_array_works() {
        assert_eq!(
            vec![
                true, false, true, true, false, true, true, false, true, false, true, false, false,
                false, false, false, false, false
            ],
            to_base64_bits("jL0")
        );
        assert_eq!(
            vec![
                true, true, true, true, false, true, true, true, true, false, false, false, false,
                false, false, false, false, false
            ],
            to_base64_bits("l70")
        );
        assert_eq!(
            vec![
                true, true, true, true, false, true, true, true, true, false, true, false, true,
                false, true, false, false, true, false, false, false, false, false, false
            ],
            to_base64_bits("lNb0")
        );
        assert_eq!(
            vec![
                true, false, true, true, false, true, true, true, true, false, false, false, true,
                true, true, true, true, false, false, true, false, true, false, true, true, false,
                false, true, true, false, true, false, false, false, true, true, false, false,
                false, true, false, false
            ],
            to_base64_bits("j7VgPn8")
        );
    }

    #[test]
    fn decode_atree_works() {
        let file = File::open("assets/atree.json")
            .expect("The file `atree.json` should exist in the folder assets.");
        let reader = BufReader::new(file);
        let abilities: AbilitiesMap = serde_json::from_reader(reader).unwrap();

        assert_eq!(
            vec!["Bash", "Spear Proficiency 1", "Cheaper Bash", "Double Bash"],
            decode_atree(&abilities.get(&Class::Warrior).unwrap(), "7")
                .iter()
                .map(|v| v.display_name.to_owned())
                .collect::<Vec<String>>()
        );
        assert_eq!(
            vec![
                "Bash",
                "Spear Proficiency 1",
                "Cheaper Bash",
                "Double Bash",
                "Charge",
                "Tougher Skin",
                "War Scream",
                "Cheaper Charge",
                "Uppercut",
            ],
            decode_atree(&abilities.get(&Class::Warrior).unwrap(), "l70")
                .iter()
                .map(|v| v.display_name.to_owned())
                .collect::<Vec<String>>()
        );
        assert_eq!(
            vec![
                "Bash",
                "Spear Proficiency 1",
                "Double Bash",
                "Charge",
                "Tougher Skin",
                "War Scream",
                "Cheaper Charge",
                "Uppercut",
                "Water Mastery",
                "Half-Moon Swipe",
                "Air Shout",
                "Generalist",
                "Cheaper Uppercut",
                "Counter",
                "Flying Kick",
                "Riposte",
                "Cheaper War Scream I",
                "Collide",
                "Whirlwind Strike",
                "Spirit of the Rabbit",
                "Cyclone",
                "Discombobulate",
                "Air Mastery"
            ],
            decode_atree(&abilities.get(&Class::Warrior).unwrap(), "j7VgPn8")
                .iter()
                .map(|v| v.display_name.to_owned())
                .collect::<Vec<String>>()
        );
    }
}

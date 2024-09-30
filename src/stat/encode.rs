pub fn encode_build(
    apparel_ids: [i32; 8],
    lvl: i32,
    weapon_id: i32,
    skillpoints: [i32; 5],
) -> String {
    let mut build_string = String::from("");

    let build_version = 8;

    // apparels
    for id in apparel_ids {
        build_string = format!("{}{}", build_string, from_int_n(id, 3));
    }
    // weapon
    build_string = format!("{}{}", build_string, from_int_n(weapon_id, 3));
    // skillpoints
    for skillpoint in skillpoints {
        build_string = format!("{}{}", build_string, from_int_n(skillpoint, 2));
    }
    // lvl
    build_string = format!("{}{}", build_string, from_int_n(lvl, 2));
    // // powders
    // build_string = format!("{}{}", build_string, "00000");
    // // tomes
    // for id in [61, 61, 62, 62, 62, 62, 63] {
    //     build_string = format!("{}{}", build_string, from_int_n(id, 2));
    // }

    format!("{}_{}", build_version, build_string)
}
const DIGITS: [char; 64] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', '+', '-',
];

fn from_int_n(int32: i32, n: u64) -> String {
    let mut result = String::new();
    let mut int32 = int32;

    for _ in 0..n {
        let digit = DIGITS[(int32 & 0x3f) as usize];
        result.insert(0, digit);
        int32 >>= 6;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encode_build_works() {
        assert_eq!(
            "8_04004B04C0482SK2SL2SM2SN03E00000000001g",
            encode_build(
                [256, 267, 268, 264, 10004, 10005, 10006, 10007],
                106,
                206,
                [0, 0, 0, 0, 0]
            )
        );
    }

    #[test]
    fn from_int_n_works() {
        assert_eq!("JI", from_int_n(1234, 2));
    }
}

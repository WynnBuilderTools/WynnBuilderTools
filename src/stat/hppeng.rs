use std::fmt::Write;

use url::Url;

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

#[cfg(test)]
mod tests {

    use super::*;

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
}

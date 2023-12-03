use common_macros::hash_map;
use cute::c;
use std::collections::HashMap;
use termimad::crossterm::style::Color::*;
use termimad::gray;
use termimad::minimad::TextTemplate;
use termimad::MadSkin;

static DESCRIPTION: &str = include_str!("day_01.md");
static EXAMPLE: &str = include_str!("example_01.txt");
static INPUT: &str = include_str!("input_01.txt");
const CALIBRATIONS: [u32; 4] = [12, 38, 15, 77];
const SUM: u32 = 142;

fn aoc_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.headers[0].set_fgbg(Black, Green);
    skin.paragraph.set_fg(gray(15));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(White);

    skin
}

fn print_template(
    tmpl: &str,
    skin: &MadSkin,
    vars: HashMap<&str, &str>,
    lines_vars: HashMap<&str, &str>,
) {
    let desc_tmpl = TextTemplate::from(tmpl);
    let mut desc_expander = desc_tmpl.expander();

    for (key, val) in vars {
        desc_expander.set(key, val);
    }

    for (key, val) in lines_vars {
        desc_expander.set_lines(key, val);
    }
    skin.print_expander(desc_expander);
}

/// Get the first number char of a str line. Returnning it as an integer.
fn first_number<I>(line_chars: I) -> Option<char>
where
    I: Iterator<Item = char>,
{
    let mut first: Option<char> = None;

    for char in line_chars {
        if char.is_ascii_digit() {
            first = Some(char);
            break;
        }
    }

    first
}

fn day_one(calibration_doc: &str) -> u32 {
    let mut all_cal: Vec<u32> = vec![];
    for line in calibration_doc.lines() {
        let first = first_number(line.chars()).unwrap();
        let last = first_number(line.chars().rev()).unwrap();
        let cal_string: String = [first, last].iter().collect();
        all_cal.push(u32::from_str_radix(&cal_string, 10).unwrap())
    }

    all_cal.iter().sum()
}

fn main() {
    let skin = aoc_skin();

    //
    let cal_str = c![cal.to_string(), for cal in CALIBRATIONS];
    let sum_str = SUM.to_string();
    print_template(
        DESCRIPTION,
        &skin,
        hash_map! {
            "cal_1" => cal_str[0].as_str(),
            "cal_2" => cal_str[1].as_str(),
            "cal_3" => cal_str[2].as_str(),
            "cal_4" => cal_str[3].as_str(),
            "sum" => sum_str.as_str()
        },
        hash_map! {
            "example" => EXAMPLE
        },
    );

    let result_one = day_one(INPUT);

    println!("The sum of the calibration values is: {}", result_one)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_01() {
        assert_eq!(day_one(EXAMPLE), SUM)
    }

    #[test]
    fn test_first_number() {
        assert_eq!(first_number("abcd".chars()), None);
        assert_eq!(first_number("1aaa".chars()), Some('1'));
        assert_eq!(first_number("a2aa".chars()), Some('2'));
        assert_eq!(first_number("aa3a".chars()), Some('3'));
        assert_eq!(first_number("aaa4".chars()), Some('4'));
        assert_eq!(first_number("1aa4".chars()), Some('1'));
        assert_eq!(first_number("1234".chars()), Some('1'));
        assert_eq!(first_number("1aa4".chars().rev()), Some('4'));
    }
}

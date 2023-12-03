use termimad::crossterm::style::Color::*;
use termimad::MadSkin;
use termimad::gray;
use termimad::minimad::TextTemplate;
use cute::c;
use common_macros::hash_map;
use std::collections::HashMap;

static DESCRIPTION: &str = include_str!("day_01.md");
static EXAMPLE: &str = include_str!("example_01.txt");
const CALIBRATIONS: [i32; 4] = [12, 38, 15, 77];
const SUM: i32 = 142;

fn aoc_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.headers[0].set_fgbg(Black, Green);
    skin.paragraph.set_fg(gray(15));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(White);

    skin
}

fn print_template(tmpl: &str, skin: &MadSkin, vars: HashMap<&str, &str>, lines_vars: HashMap<&str, &str>) {
    let desc_tmpl= TextTemplate::from(tmpl);
    let mut desc_expander = desc_tmpl.expander();

    for (key, val) in vars {
        desc_expander.set(key, val);    
    }

    for (key, val) in lines_vars {
        desc_expander.set_lines(key, val);    
    }
    skin.print_expander(desc_expander);
}

fn main() {
    let skin = aoc_skin();

    // 
    let cal_str = c![cal.to_string(), for cal in CALIBRATIONS];
    let sum_str = SUM.to_string();
    print_template(
        DESCRIPTION,
        &skin,
        hash_map!{
            "cal_1" => cal_str[0].as_str(),
            "cal_2" => cal_str[1].as_str(),
            "cal_3" => cal_str[2].as_str(),
            "cal_4" => cal_str[3].as_str(),
            "sum" => sum_str.as_str()
        },
        hash_map!{
            "example" => EXAMPLE
        }
    );
}

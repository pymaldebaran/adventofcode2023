use termimad::crossterm::style::Color::*;
// use termimad::crossterm::style::Attribute::*;
use termimad::MadSkin;
use termimad::gray;

static DESCRIPTION: &str = include_str!("day_01.md");

fn main() {
    let mut skin = MadSkin::default();
    skin.headers[0].set_fgbg(Black, Green);
    skin.paragraph.set_fg(gray(15));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(White);

    skin.print_text(DESCRIPTION);
}

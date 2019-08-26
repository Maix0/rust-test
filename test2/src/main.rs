fn main() {
    
    println!("{}",color("Colored String O_o", Color::Blue, Color::Black, vec![Effect::Blink]));
    println!("{}",color("Colored String O_o", Color::Cyan, Color::Black, vec![Effect::Bold]));
    println!("{}",color("Colored String O_o", Color::Red, Color::Black, vec![Effect::None]));
    println!("{}",color("Colored StringV2", Color::BrightBlue,Color::Black,vec![Effect::Bold, Effect::Underline]));
    println!("{}",color("Colored StringV2", Color::BrightCyan,Color::Black, vec![Effect::None]));
    println!("{}",color("Colored StringV2", Color::BrightRed,Color::Black, vec![Effect::Blink, Effect::Bold, Effect::Underline]));
}

enum Color  {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightPurple,
    BrightCyan,
    BrightWhite,
}

enum Effect {
    None = 0,
    Bold = 1,
    Underline = 4,
    Blink = 5,
}

fn color(base:&str,color:Color,background: Color, style:Vec<Effect>) -> String {
    let color_int:u32 = match color {
        Color::Black            => 30,
        Color::Red              => 31,
        Color::Green            => 32,
        Color::Yellow           => 33,
        Color::Blue             => 34,
        Color::Purple           => 35,
        Color::Cyan             => 36,
        Color::White            => 37,
        Color::BrightBlack      => 90,
        Color::BrightRed        => 91,
        Color::BrightGreen      => 92,
        Color::BrightYellow     => 93,
        Color::BrightBlue       => 94,
        Color::BrightPurple     => 95,
        Color::BrightCyan       => 96,
        Color::BrightWhite      => 97,
    };
    let bg_int : u32 = match background {
        Color::Black            => 40,
        Color::Red              => 41,
        Color::Green            => 42,
        Color::Yellow           => 43,
        Color::Blue             => 44,
        Color::Purple           => 45,
        Color::Cyan             => 46,
        Color::White            => 47,
        Color::BrightBlack      => 100,
        Color::BrightRed        => 101,
        Color::BrightGreen      => 102,
        Color::BrightYellow     => 103,
        Color::BrightBlue       => 104,
        Color::BrightPurple     => 105,
        Color::BrightCyan       => 106,
        Color::BrightWhite      => 107,

    };
    let mut effects: String = String::from("");
    for effect in style {
        effects = format!("{};{}", effects, match effect {
            Effect::None => 0,
            Effect::Bold => 1,
            Effect::Underline => 4,
            Effect::Blink => 5,
        });
    }

return format!("\x1b[{};{}{}m{}\x1b[0m",color_int,bg_int,effects,base);
}

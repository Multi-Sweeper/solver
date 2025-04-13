use nu_ansi_term::{Color, Style};

pub fn colour_str(str: &str, rgb: (u8, u8, u8), background: Option<(u8, u8, u8)>) -> String {
    // Rgb(rgb.0, rgb.1, rgb.2).paint(str).to_string()
    let mut style = Style::new();
    style.foreground = Some(Color::Rgb(rgb.0, rgb.1, rgb.2));
    if let Some((r, g, b)) = background {
        style.background = Some(Color::Rgb(r, g, b));
    }

    style.paint(str).to_string()

    // let builder = AnsiBuilder::new()
    //     .color()
    //     .fg()
    //     .rgb(rgb.0, rgb.1, rgb.2)
    //     .text(str);

    // builder.0;

    // format!(
    //     "\u{001B}[38;2;{};{};{}m{str}\u{001B}[0m",
    //     rgb.0, rgb.1, rgb.2
    // )
}

pub trait Coloured {
    fn to_coloured(&self, background: Option<(u8, u8, u8)>) -> String;
}

impl Coloured for String {
    fn to_coloured(&self, background: Option<(u8, u8, u8)>) -> String {
        match self.as_str() {
            "1" => colour_str(self, (0, 120, 255), background),
            "2" => colour_str(self, (0, 255, 0), background),
            "3" => colour_str(self, (255, 0, 0), background),
            "4" => colour_str(self, (0, 0, 255), background),
            "5" => colour_str(self, (150, 0, 0), background),
            "6" => colour_str(self, (0, 130, 130), background),
            "7" => colour_str(self, (100, 100, 100), background),
            "8" => colour_str(self, (0, 0, 0), background),
            "B" => colour_str(self, (0, 0, 0), background),
            "F" => colour_str(self, (255, 50, 50), background),
            "?" => colour_str(self, (150, 150, 150), background),
            "*" => colour_str(self, (150, 150, 0), background),
            _ => colour_str(self, (255, 255, 255), background),
        }
    }
}

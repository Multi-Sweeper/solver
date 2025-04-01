pub fn colour_str(str: &str, rgb: (u8, u8, u8)) -> String {
    format!(
        "\u{001B}[38;2;{};{};{}m{str}\u{001B}[0m",
        rgb.0, rgb.1, rgb.2
    )
}

pub trait Coloured {
    fn to_coloured(&self) -> String;
}

impl Coloured for String {
    fn to_coloured(&self) -> String {
        match self.as_str() {
            "1" => colour_str(self, (0, 120, 255)),
            "2" => colour_str(self, (0, 255, 0)),
            "3" => colour_str(self, (255, 0, 0)),
            "4" => colour_str(self, (0, 0, 255)),
            "5" => colour_str(self, (150, 0, 0)),
            "6" => colour_str(self, (0, 130, 130)),
            "7" => colour_str(self, (100, 100, 100)),
            "8" => colour_str(self, (0, 0, 0)),
            "B" => colour_str(self, (0, 0, 0)),
            "F" => colour_str(self, (255, 50, 50)),
            "?" => colour_str(self, (150, 150, 150)),
            "*" => colour_str(self, (150, 150, 0)),
            _ => self.to_string(),
        }
    }
}

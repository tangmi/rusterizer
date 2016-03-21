use sdl2::pixels::Color;

#[derive(Clone, Copy, Default)]
pub struct Rgb24 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Color> for Rgb24 {
    fn from(color: Color) -> Self {
        let (r, g, b) = color.rgb();
        Rgb24 { r: r, g: g, b: b }
    }
}

//impl Default for Rgb24 {
//    fn default() -> Rgb24 {
//        Rgb24 { r: 0, g: 0, b: 0 }
//    }
//}

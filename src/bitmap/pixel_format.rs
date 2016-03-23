use sdl2::pixels::Color;
use math::Clamp;

pub trait TransferToRgb {
    fn transfer(&self) -> (u8, u8, u8);
}

#[derive(Clone, Copy)]
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

impl Default for Rgb24 {
    fn default() -> Rgb24 {
        Rgb24 { r: 0, g: 0, b: 0 }
    }
}

impl TransferToRgb for Rgb24 {
    fn transfer(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl TransferToRgb for f64 {
    fn transfer(&self) -> (u8, u8, u8) {
        let val = 255 - (((*self - 10.0) * 3.0).clamp(0.0, 1.0) * 255.0) as u8;
        (val, val, val)
    }
}

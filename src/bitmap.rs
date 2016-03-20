
use sdl2::pixels::Color;

use cgmath::Vector2;

pub struct Bitmap {
    buffer: Vec<u8>,
    width: u32,
    height: u32,
}

impl Bitmap {
    pub fn new(width: u32, height: u32) -> Bitmap {
        let len = (width * height * 3) as usize;
        let mut buffer = Vec::with_capacity(len);
        for _ in 0..len {
            buffer.push(0); // todo(tang): this sucks
        }

        Bitmap {
            buffer: buffer,
            width: width,
            height: height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn slice(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    pub fn clear(&mut self, color: Color) {
        let (r, g, b) = color.rgb();
        for y in 0..self.height {
            for x in 0..self.width {
                let offset = ((y * self.width + x) * 3) as usize;
                self.buffer[offset + 0] = r;
                self.buffer[offset + 1] = g;
                self.buffer[offset + 2] = b;
            }
        }
    }

    pub fn set_pixel(&mut self, point: Vector2<u32>, color: Color) {
        let offset = ((point.y * self.width + point.x) * 3) as usize;

        let (r, g, b) = color.rgb();

        self.buffer[offset + 0] = r;
        self.buffer[offset + 1] = g;
        self.buffer[offset + 2] = b;
    }
}

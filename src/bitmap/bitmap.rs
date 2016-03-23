use std::mem;
use cgmath::Vector2;

pub struct Bitmap<T> {
    buffer: Vec<T>,
    width: u32,
    height: u32,
}

impl<T> Bitmap<T>
    where T: Default + Copy
{
    pub fn new(width: u32, height: u32) -> Bitmap<T> {
        let size = mem::size_of::<T>();
        let len = (width * height * size as u32) as usize;
        let mut buffer = Vec::with_capacity(len);

        for _ in 0..len {
            buffer.push(T::default()); // todo(tang): this sucks
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

    pub fn slice(&self) -> &[T] {
        self.buffer.as_slice()
    }

    pub fn clear(&mut self, t: T) {
        for y in 0..self.height {
            for x in 0..self.width {
                let offset = (y * self.width + x) as usize;
                self.buffer[offset] = t;
            }
        }
    }

    pub fn set_pixel(&mut self, point: Vector2<u32>, t: T) {
        let offset = (point.y * self.width + point.x) as usize;
        self.buffer[offset] = t;
    }
    
    pub fn get_pixel(&self, point: Vector2<u32>) -> T {
    	let offset = (point.y * self.width + point.x) as usize;
        self.buffer[offset]
    }
}

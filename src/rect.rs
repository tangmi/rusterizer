use cgmath::Vector2;
use std::i32;

pub type Point = Vector2<i32>;

#[derive(Debug)]
pub struct Rect {
    pub top: i32,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
}

impl Rect {
    fn new_unsafe(top: i32, right: i32, bottom: i32, left: i32) -> Rect {
        Rect {
            top: top,
            left: left,
            bottom: bottom,
            right: right,
        }
    }

    fn is_valid(&self) -> bool {
        self.top <= self.bottom &&
        self.left <= self.right
    }

    // pub fn new_bounding_points(top_left: Point, bottom_right: Point) -> Rect {
    //     // todo validate?
    //     Rect {
    //         top_left: top_left,
    //         bottom_right: bottom_right,
    //     }
    // }

    pub fn from_bounding(pts: &[Point]) -> Rect {
        let rect = pts.iter().fold(
            Rect::new_unsafe(i32::max_value(), i32::min_value(), i32::min_value(), i32::max_value()),
            |mut acc, &pt| {
                if pt.x < acc.left {
                    acc.left = pt.x;
                }
                if pt.x > acc.right {
                    acc.right = pt.x;
                }
                if pt.y < acc.top {
                    acc.top = pt.y;
                }
                if pt.y > acc.bottom {
                    acc.bottom = pt.y;
                }
                return acc;
            }
        );
        assert!(rect.is_valid());
        rect
    }
}

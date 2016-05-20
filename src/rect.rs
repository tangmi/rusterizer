use cgmath::Vector2;
use std::i32;
use std::cmp;

pub type Point = Vector2<i32>;

#[derive(Debug)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub bottom: i32,
    pub right: i32,
}

impl Rect {
    fn new_unsafe(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
        Rect {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
        }
    }

    pub fn new(top_left: Point, bottom_right: Point) -> Rect {
        assert!(top_left.x <= bottom_right.x);
        assert!(top_left.y <= bottom_right.y);
        Rect::new_unsafe(top_left.x, top_left.y, bottom_right.x, bottom_right.y)
    }

    pub fn intersect(&self, other: Rect) -> Option<Rect> {
        let rect = Rect::new_unsafe(
            cmp::max(self.left, other.left),
            cmp::max(self.top, other.top),
            cmp::min(self.right, other.right),
            cmp::min(self.bottom, other.bottom)
        );

        if self.right < other.left
        || self.bottom < other.top
        || other.right < self.left
        || other.bottom < self.top {
            Option::None
        } else {
            Option::Some(rect)
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
            Rect::new_unsafe(
                i32::max_value(),
                i32::max_value(),
                i32::min_value(),
                i32::min_value()
            ),
            |acc, &pt| Rect::new_unsafe(
                    cmp::min(pt.x, acc.left),
                    cmp::min(pt.y, acc.top),
                    cmp::max(pt.x, acc.right),
                    cmp::max(pt.y, acc.bottom)
                )
            );
        assert!(rect.is_valid());
        rect
    }
}

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

pub fn min(of: i32, or: i32) -> i32 {
    of.min(or)
}

pub fn max(of: i32, or: i32) -> i32 {
    of.max(or)
}

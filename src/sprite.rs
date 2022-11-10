use crate::common::{max, min, Size, Vec2};
use simple_pixels::{rgb::RGBA8, Context};

pub struct Sprite {
    pub origin: Vec2,
    pub size: Size,
    pub pixels: Vec<RGBA8>,
}

impl Sprite {
    pub fn new(pos: Vec2, size: Size, pixels: Vec<RGBA8>) -> Self {
        Self {
            origin: pos,
            size,
            pixels,
        }
    }
    pub fn draw(&self, ctx: &mut Context) {
        let screen_width = ctx.width();
        let screen_height = ctx.height();

        let screen_size = Size::new(screen_width, screen_height);
        let screen_origin = Vec2::new(0.0, 0.0);

        let visible_from_x = max(screen_origin.x as i32, self.origin.x as i32);
        let visible_to_x = min(
            self.size.width as i32 + self.origin.x as i32,
            screen_size.width as i32 + screen_origin.x as i32,
        );
        let visible_width = visible_to_x - visible_from_x;
        let sprite_offset_x = if self.origin.x < screen_origin.x {
            -(self.origin.x - screen_origin.x) as i32
        } else {
            0
        };

        let visible_from_y = max(screen_origin.y as i32, self.origin.y as i32);
        let visible_to_y = min(
            self.size.height as i32 + self.origin.y as i32,
            screen_size.height as i32 + screen_origin.y as i32,
        );
        let visible_height = visible_to_y - visible_from_y;
        let sprite_offset_y = if self.origin.y < screen_origin.y {
            -(self.origin.y - screen_origin.y) as i32
        } else {
            0
        };

        let mut visible_pixels: Vec<RGBA8> =
            Vec::with_capacity((visible_width * visible_height).try_into().unwrap());

        for y in sprite_offset_y..visible_height + sprite_offset_y {
            for x in sprite_offset_x..visible_width + sprite_offset_x {
                visible_pixels.push(self.pixels[((y * self.size.width as i32) + x) as usize]);
            }
        }

        ctx.draw_pixels(
            max(screen_origin.x as i32, self.origin.x as i32),
            max(screen_origin.y as i32, self.origin.y as i32),
            visible_width.try_into().unwrap(),
            visible_height.try_into().unwrap(),
            &visible_pixels,
        );
    }
}

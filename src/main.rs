#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::missing_const_for_fn,
    clippy::cast_possible_wrap,
    clippy::missing_panics_doc
)]
use simple_pixels::{rgb::RGBA8, start, Config, Context, KeyCode, MouseButton, State};

use std::fs::write;
use std::path::{Path, PathBuf};

mod cli;
mod clock;
mod common;
mod ppt;
mod sprite;

use cli::Arguments;
use clock::Clock;
use common::{Size, Vec2};
use ppt::{load_sprite, save_sprite};
use sprite::Sprite;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 480;

fn main() {
    let args = Arguments::new();

    assert!(args.unnamed.len() == 1, "Expecting a file path to edit.");
    let scale = args
        .named
        .get("scale")
        .map(|arg| {
            arg.parse::<u32>()
                .expect("Couldn't parse scale as an integer!")
        })
        .unwrap_or_else(|| 10);
    let width = args
        .named
        .get("width")
        .map(|arg| {
            arg.parse::<u32>()
                .expect("Couldn't parse width as an integer!")
        })
        .unwrap_or_else(|| WIDTH);
    let height = args
        .named
        .get("height")
        .map(|arg| {
            arg.parse::<u32>()
                .expect("Couldn't parse height as an integer!")
        })
        .unwrap_or_else(|| HEIGHT);
    let file_path = Path::new(args.unnamed.first().unwrap()).to_owned();
    let config = Config {
        window_title: "game".to_string(),
        window_width: width * scale,
        window_height: height * scale,
        fullscreen: false,
        icon: None,
    };

    let game = Game::new(file_path, scale, Size::new(width, height));
    start(config, game);
}

struct Game {
    clock: Clock,
    canvas: Sprite,
    scale: u32,
    size: Size,
    path: PathBuf,
}

impl Game {
    pub fn new(file_path: PathBuf, scale: u32, size: Size) -> Self {
        let mut canvas = load_sprite(&file_path).unwrap_or_else(|_| {
            let pixels: Vec<RGBA8> = vec![RGBA8::default(); size.area()];
            let canvas = Sprite::new(Vec2::new(0.0, 0.0), size, pixels);
            canvas
        });
        canvas = if canvas.size.area() != size.area() {
            let pixels: Vec<RGBA8> = vec![RGBA8::default(); size.area()];
            let canvas = Sprite::new(Vec2::new(0.0, 0.0), size, pixels);
            canvas
        } else {
            canvas
        };
        let clock = Clock::new();
        Self {
            clock,
            canvas,
            scale,
            size,
            path: file_path,
        }
    }
}

impl State for Game {
    fn update(&mut self, ctx: &mut Context) {
        let (mut r, mut g, mut b) = (50, 100, 255);
        if ctx.is_key_down(KeyCode::Escape) {
            self.save();
            ctx.quit();
        }

        let (x, y) = ctx.get_mouse_pos();
        let (grid_x, grid_y) = (x as u32 / self.scale, y as u32 / self.scale);
        let index = (grid_y * self.canvas.size.width + grid_x) as usize;
        if ctx.is_mouse_button_down(MouseButton::Right) {
            self.canvas.pixels[index] = RGBA8::default();
        }
        if ctx.is_mouse_button_down(MouseButton::Left) {
            self.canvas.pixels[index] = RGBA8::new(r, g, b, 255);
        }

        self.clock.sleep();
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear();

        for y in 0..self.canvas.size.height - 1 {
            for x in 0..self.canvas.size.width - 1 {
                let index = (y * self.canvas.size.width + x) as usize;
                let pixel = self.canvas.pixels[index];
                ctx.draw_rect(
                    (x * self.scale).try_into().unwrap(),
                    (y * self.scale).try_into().unwrap(),
                    self.scale,
                    self.scale,
                    pixel,
                );
            }
        }
    }
}

impl Game {
    fn save(&self) {
        save_sprite(&self.path, &self.canvas);
    }
}

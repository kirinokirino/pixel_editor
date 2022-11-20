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
mod font;
mod ppt;
mod sprite;

use cli::Arguments;
use clock::Clock;
use common::{constrain, Size, Vec2};
use font::Font;
use ppt::{load_sprite, save_sprite};
use sprite::Sprite;

const WIDTH: u32 = 40;
const HEIGHT: u32 = 30;

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

enum Selection {
    R,
    G,
    B,
}

struct Game {
    clock: Clock,
    canvas: Sprite,
    scale: u32,
    size: Size,
    path: PathBuf,
    font: Font,
    selected_color: RGBA8,
    selection: Selection,
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
        let font = Font::new();
        let selected_color = RGBA8::new(100, 100, 100, 255);
        Self {
            clock,
            canvas,
            scale,
            size,
            path: file_path,
            font,
            selected_color,
            selection: Selection::R,
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
        let (grid_x, grid_y) = (
            constrain(x as u32, 0, (self.size.width - 1) * self.scale) as u32 / self.scale,
            constrain(y as u32, 0, (self.size.height - 1) * self.scale) as u32 / self.scale,
        );
        let index = (grid_y * self.canvas.size.width + grid_x) as usize;
        if ctx.is_mouse_button_down(MouseButton::Right) {
            self.canvas.pixels[index] = RGBA8::default();
        }
        if ctx.is_mouse_button_down(MouseButton::Left) {
            self.canvas.pixels[index] = self.selected_color;
        }
        if ctx.is_key_pressed(KeyCode::O) {
            self.selection_decrease();
        } else if ctx.is_key_pressed(KeyCode::U) {
            self.selection_increase();
        } else if ctx.is_key_pressed(KeyCode::Period) {
            self.color_increase();
        } else if ctx.is_key_pressed(KeyCode::E) {
            self.color_decrease();
        }

        self.clock.sleep();
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear();

        for y in 0..self.canvas.size.height {
            for x in 0..self.canvas.size.width {
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
        self.display_selected_color(ctx);
    }
}

impl Game {
    fn save(&self) {
        save_sprite(&self.path, &self.canvas);
    }

    fn display_selected_color(&mut self, ctx: &mut Context) {
        let RGBA8 { r, g, b, a } = self.selected_color;
        let (mut sr, mut sg, mut sb) = (' ',' ',' ');
        match self.selection {
        	Selection::R => { sr = '>'},
        	Selection::G => { sg = '>'},
        	Selection::B => { sb = '>'},
        }
        let display_str = format!("color:{sr}r:{r},{sg}g:{g},{sb}b:{b}");
        let pos = Vec2::new(10.0, (self.size.height * self.scale) as f32 - 20.0);
        self.font.draw(ctx, &display_str, pos);
    }

    fn selection_increase(&mut self) {
        self.selection = match self.selection {
            Selection::R => Selection::G,
            Selection::G => Selection::B,
            Selection::B => Selection::R,
        }
    }
    
    fn selection_decrease(&mut self) {
        self.selection = match self.selection {
            Selection::R => Selection::B,
            Selection::G => Selection::R,
            Selection::B => Selection::G,
        }
    }
    fn color_increase(&mut self) {
        let RGBA8 {
            mut r,
            mut g,
            mut b,
            a,
        } = self.selected_color;

        match self.selection {
            Selection::R => {
                r = r.saturating_add(10);
            }
            Selection::G => {
                g = g.saturating_add(10);
            }
            Selection::B => {
                b = b.saturating_add(10);
            }
        }
        self.selected_color = RGBA8::new(r, g, b, a);
    }
    fn color_decrease(&mut self) {
        let RGBA8 {
            mut r,
            mut g,
            mut b,
            a,
        } = self.selected_color;

        match self.selection {
            Selection::R => {
                r = r.saturating_sub(10);
            }
            Selection::G => {
                g = g.saturating_sub(10);
            }
            Selection::B => {
                b = b.saturating_sub(10);
            }
        }
        self.selected_color = RGBA8::new(r, g, b, a);}
}

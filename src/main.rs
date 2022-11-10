#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::missing_const_for_fn,
    clippy::cast_possible_wrap,
    clippy::missing_panics_doc
)]
use simple_pixels::{start, Config, Context, KeyCode, State};

mod cli;
mod clock;
mod common;
mod ppt;
mod settings;
mod sprite;

use cli::Arguments;
use clock::Clock;
use common::{Size, Vec2};
use ppt::load_sprite;
use sprite::Sprite;

fn main() {
    let config = Config {
        window_title: "game".to_string(),
        window_width: 200,
        window_height: 200,
        fullscreen: false,
        icon: None,
    };

    let game = Game::new();
    start(config, game);
}

struct Game {
    clock: Clock,
    sprite: Sprite,
}

impl Game {
    pub fn new() -> Self {
        let sprite = load_sprite("test.ppt").unwrap();
        let clock = Clock::new();
        Self { clock, sprite }
    }
}

impl State for Game {
    fn update(&mut self, ctx: &mut Context) {
        if ctx.is_key_down(KeyCode::Escape) {
            ctx.quit();
        }

        let mouse = ctx.get_mouse_pos();
        self.sprite.origin = Vec2::new(mouse.0, mouse.1);

        self.clock.sleep();
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear();
        self.sprite.draw(ctx);
    }
}

use std::cmp::min;
use std::fs;
use std::io;
use std::mem;
use std::path::Path;

use simple_pixels::{rgb::RGBA8, Context};

use crate::common::Vec2;

const CHAR_WIDTH: usize = 9;
const CHAR_HEIGHT: usize = 14;

struct LetterSprite {
    pub pixels: [RGBA8; CHAR_WIDTH * CHAR_HEIGHT],
}

impl LetterSprite {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let data = fs::read_to_string(path)?;
        if let Some((header, data)) = data.split_once('\n') {
            let data: Vec<&str> = data.split_ascii_whitespace().collect();
            let mut split = header.split_whitespace();
            let format = split
                .next()
                .expect("Error while parsing ppt sprite: no format!");
            assert!(format == "P3", "Only support P3 ppt version");
            let width = split
                .next()
                .expect("Error while parsing ppt sprite: no sprite width!")
                .parse::<usize>()
                .expect("couldn't parse sprite width");
            let height = split
                .next()
                .expect("Error while parsing ppt sprite: no sprite height!")
                .parse::<usize>()
                .expect("couldn't parse sprite height");
            let colors = split
                .next()
                .expect("Error while parsing ppt sprite: no max colors!");
            assert!(colors == "255", "Not yet support anything but 255 colors");
            assert!(split.next().is_none(), "Unknown additional header fields");

            assert!(width == CHAR_WIDTH);
            assert!(height == CHAR_HEIGHT);
            let mut p: [mem::MaybeUninit<RGBA8>; CHAR_HEIGHT * CHAR_WIDTH] =
                unsafe { mem::MaybeUninit::uninit().assume_init() };
            for y in 0..height {
                for x in 0..width {
                    let r = data[y * width * 3 + x * 3];
                    let g = data[y * width * 3 + x * 3 + 1];
                    let b = data[y * width * 3 + x * 3 + 2];
                    p[y * width + x] = mem::MaybeUninit::new(RGBA8::new(
                        r.parse::<u8>().unwrap(),
                        g.parse::<u8>().unwrap(),
                        b.parse::<u8>().unwrap(),
                        0,
                    ));
                }
            }
            let pixels = unsafe { mem::transmute::<_, [RGBA8; CHAR_HEIGHT * CHAR_WIDTH]>(p) };
            return Ok(Self { pixels });
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Couldn't split file on newline!",
        ))
    }
}

const ASCII: &str = "!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
pub struct Font {
    letters: Vec<LetterSprite>,
}

impl Font {
    pub fn new() -> Self {
        let mut letters = Vec::with_capacity(ASCII.len());
        const FILE_EXTENSION: &str = ".ppt";
        const FONT_FOLDER: &str = "font/";
        for (_i, letter) in ASCII.chars().enumerate() {
            let file_name: String = if letter == '/' {
                "slash".to_string()
            } else {
                letter.to_string()
            };
            let file_path = format!("{FONT_FOLDER}{file_name}{FILE_EXTENSION}");
            letters.push(LetterSprite::new(file_path).unwrap());
        }
        Self { letters }
    }

    pub fn letter(&self, ch: char) -> &LetterSprite {
        &self.letters[ch as usize - 33]
    }

    pub fn index(&self, ch: char) -> usize {
        ch as usize - 33
    }

    pub fn draw(&self, ctx: &mut Context, text: &str, origin: Vec2) {
        let max_pos = Vec2::new(ctx.width() as f32, ctx.height() as f32);
        for (y, line) in text.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let pos_y = origin.y + (y * CHAR_HEIGHT) as f32;
                if (pos_y >= max_pos.y) {
                    return;
                }
                let pos_x = origin.x + (x * CHAR_WIDTH) as f32;
                if (pos_x >= max_pos.x) {
                    continue;
                }
                let pos = Vec2::new(pos_x, pos_y);
                self.draw_char(ctx, ch, pos);
            }
        }
    }
    pub fn draw_char(&self, ctx: &mut Context, ch: char, pos: Vec2) {
        if (ch.is_whitespace()) {
            return;
        }
        for char_y in 0..CHAR_HEIGHT {
            for char_x in 0..CHAR_WIDTH {
                let letter = self.letter(ch);
                ctx.draw_pixels(
                    pos.x as u32,
                    pos.y as u32,
                    CHAR_WIDTH as u32,
                    CHAR_HEIGHT as u32,
                    &letter.pixels,
                );
            }
        }
    }
}

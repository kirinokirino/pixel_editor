use std::fs;
use std::io;
use std::path::Path;

use simple_pixels::rgb::RGBA8;

use crate::{Size, Sprite, Vec2};

/// Basic image saving/loading with ppt format.

pub fn save_sprite(path: &str, sprite: &Sprite) -> Result<(), io::Error> {
    let path = Path::new(path);

    let (width, height) = (sprite.size.width, sprite.size.height);
    let mut data: String = String::with_capacity(sprite.pixels.len() * 3 * 4); // 3 colors, 4 chars per color string (max)
    for pixel in &sprite.pixels {
        data.push_str(&format!("{} {} {} ", pixel.r, pixel.g, pixel.b));
    }
    let ppt_header = format!("P3 {width} {height} 255\n");
    fs::write(path, [ppt_header, data].concat())
}

pub fn load_sprite(path: &str) -> Result<Sprite, io::Error> {
    let path = Path::new(path);
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

        let mut pixels: Vec<RGBA8> = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let r = data[y * width * 3 + x * 3];
                let g = data[y * width * 3 + x * 3 + 1];
                let b = data[y * width * 3 + x * 3 + 2];
                let a = 255;
                pixels.push(RGBA8::new(
                    r.parse::<u8>().unwrap(),
                    g.parse::<u8>().unwrap(),
                    b.parse::<u8>().unwrap(),
                    a,
                ));
            }
        }

        return Ok(Sprite::new(
            Vec2::new(0.0, 0.0),
            Size::new(width.try_into().unwrap(), height.try_into().unwrap()),
            pixels,
        ));
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Couldn't split file on newline!",
    ))
}

/*

fn resize_bitmap(bitmap: Vec<u8>, width: u32, height: u32) -> BitMap {
    let word_width = 1 + (width / 32);
    let byte_width = 1 + (width / 8);

    let mut resized_bitmap = BitMap::new(word_width, height);

    for (i, row) in bitmap
        .chunks_exact(byte_width as usize)
        .chain(
            [0u8, 0u8, 0u8, 0u8]
                .chunks_exact(byte_width as usize)
                .cycle(),
        )
        .enumerate()
    {
        if i == height as usize {
            break;
        }
        let mut word = 0u32;
        for (j, byte) in row.iter().enumerate() {
            match j {
                3 => word = word | (0x000000FF & (*byte as u32)),
                2 => word = word | (0x0000FF00 & (*byte as u32) << 8),
                1 => word = word | (0x00FF0000 & (*byte as u32) << 16),
                0 => word = word | (0xFF000000 & (*byte as u32) << 24),
                _ => panic!(),
            }
        }
        resized_bitmap.data[i] = word;
    }
    resized_bitmap
}

#[derive(Debug)]
pub struct BitMap {
    height: u32,
    width: u32,
    data: Vec<u32>,
}

impl BitMap {
    pub fn new(width: u32, height: u32) -> Self {
        let size = height * ((width % 32) + 1);
        let buffer = vec![0; size as usize];
        Self {
            height,
            width,
            data: buffer,
        }
    }

    fn buffer_size(&self) -> u32 {
        self.height * ((self.width % 32) + 1)
    }

    fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = y * (1 + (self.width % 32)) + x;
        if offset < self.buffer_size() {
            Some(offset as usize)
        } else {
            None
        }
    }

    pub fn get_byte(&self, x: u32, y: u32) -> Option<u32> {
        match self.get_offset(x, y) {
            Some(offset) => Some(self.data[offset]),
            None => None,
        }
    }

    pub fn set_byte(&mut self, x: u32, y: u32, byte: u32) -> bool {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.data[offset] = byte;
                true
            }
            None => false,
        }
    }

    pub fn write_file(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let header = format!("P1 {} {}\n", self.width * 32, self.height);
        file.write(header.as_bytes())?;
        file.write(&self.data().as_bytes())?;
        Ok(())
    }

    pub fn data(&self) -> String {
        let res = self
            .data
            .iter()
            .enumerate()
            .map(|(i, word)| {
                let newline = match (i + 1) % (self.width as usize) {
                    0 => "\n",
                    _ => "",
                };

                format!("{:0>32b}{}", word, newline)
            })
            .collect();
        res
    }
}

*/


#![allow(dead_code)]
#![allow(unused_variables)]

use sdl2::rect::Rect;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{TextureCreator,Texture,BlendMode};


pub const GLYPH_COUNT: u32 = 256;

const GLYPH_WIDTH: u32 = 8;
const GLYPHS: &[u8] = include_bytes!("../COMPUTER.F14");

pub trait StrAsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl StrAsBytes for str {
    fn as_bytes(&self) -> &[u8] {
        str::as_bytes(self)
    }
}

impl StrAsBytes for [u8] {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

pub struct GlyphAtlas<'a> {
    texture: Texture<'a>,
    glyph_height: u32,
    glyph_width: u32,
}

impl<'a> GlyphAtlas<'a> {
    pub fn new<T>(texture_creator: &TextureCreator<T>) -> GlyphAtlas {
        let atlas_width = GLYPH_WIDTH;
        let atlas_height = GLYPHS.len() as u32;
        let glyph_height = GLYPHS.len() as u32 / GLYPH_COUNT;
        let mut buffer: Vec<u8> = Vec::with_capacity((atlas_width * atlas_height * 4) as usize);
        let mut atlas = texture_creator.create_texture_static(PixelFormatEnum::ABGR8888, atlas_width, atlas_height).expect("Could not create texture");
        for byte in GLYPHS {
            for i in 0..8 {
                let value = if byte & (0b1000_0000 >> i) != 0 {
                    255
                } else {
                    0
                };
                buffer.push(value);
                buffer.push(value);
                buffer.push(value);
                buffer.push(value);
            }
        }
        atlas.update(None, &buffer[..], (atlas_width * 4) as usize).expect("Could not set texture data");
        atlas.set_blend_mode(BlendMode::Blend);
        GlyphAtlas {
            texture: atlas,
            glyph_width: GLYPH_WIDTH,
            glyph_height,
        }
    }

    pub fn glyph_rect(&self, glyph: u8) -> Rect {
        Rect::new(0, self.glyph_height as i32 * glyph as i32, self.glyph_width, self.glyph_height)
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn set_render_color(&mut self, r: u8, g: u8, b: u8) {
        self.texture.set_color_mod(r, g, b)
    }

    /*pub fn text_rects(&'a self, text: &'a str, position: (i32, i32), scale: u32) -> GlyphIterator<'a> {
        self.text_rects_bytes(text.as_bytes(), position, scale)
        /*GlyphIterator {
            atlas: self,
            chars: text.as_bytes(),
            scale,
            position
        }*/
    }*/

    pub fn text_rects<T: StrAsBytes + ?Sized>(&'a self, text: &'a T, position: (i32, i32), scale: u32) -> GlyphIterator<'a> {
        GlyphIterator {
            atlas: self,
            chars: text.as_bytes(),
            scale,
            position
        }
    }

    pub fn text_rects_centered<T: StrAsBytes + ?Sized>(&'a self, text: &'a T, position: (i32, i32), scale: u32) -> GlyphIterator<'a> {
        let x = position.0 - text_width(text, scale) / 2;
        let y = position.1 - (self.glyph_height as i32 * scale as i32 / 2);
        self.text_rects(text, (x, y), scale)
    }

    pub fn text_rects_right_aligned<T: StrAsBytes + ?Sized>(&'a self, text: &'a T, position: (i32, i32), scale: u32) -> GlyphIterator<'a> {
        let x = position.0 - text_width(text, scale);
        let y = position.1;
        self.text_rects(text, (x, y), scale)
    }

}

fn text_width<T: StrAsBytes + ?Sized>(text: &T, scale: u32) -> i32 {
    text.as_bytes().len() as i32 * GLYPH_WIDTH as i32 * scale as i32 + (text.as_bytes().len() - 1) as i32 * scale as i32
}

pub struct GlyphIterator<'a> {
    atlas: &'a GlyphAtlas<'a>,
    chars: &'a [u8],
    scale: u32,
    position: (i32, i32),
}

impl<'a> Iterator for GlyphIterator<'a> {
    type Item = (Rect, Rect);

    fn next(&mut self) -> Option<(Rect, Rect)> {
        if self.chars.is_empty() {
            return None;
        }
        let src_rect = self.atlas.glyph_rect(self.chars[0]);
        let mut dst_rect = src_rect.clone();
        let w = dst_rect.width() * self.scale;
        let h = dst_rect.height() * self.scale;
        dst_rect.set_width(w);
        dst_rect.set_height(h);
        dst_rect.set_x(self.position.0);
        dst_rect.set_y(self.position.1);
        self.position.0 += dst_rect.width() as i32 + self.scale as i32;
        self.chars = &self.chars[1..];
        Some((src_rect, dst_rect))
    }
}

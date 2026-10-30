//! Software-only indexed sprite renderer. Every visible pixel comes from a hand-authored sprite or font glyph.
use crate::assets::{self, Sprite, W, H};
use std::sync::OnceLock;
static FONT: OnceLock<std::collections::BTreeMap<char, [&'static str; 7]>> = OnceLock::new();

pub struct Renderer { pub pixels: Vec<u32> }
impl Renderer {
    pub fn new() -> Self { Self { pixels: vec![assets::PALETTE[0]; W*H] } }
    pub fn clear_with(&mut self, index: usize) { self.pixels.fill(assets::PALETTE[index]); }
    pub fn blit(&mut self, sprite: &Sprite, x: i32, y: i32, flip_x: bool, swap: Option<(usize,usize)>) {
        let width = sprite.width();
        for (sy,row) in sprite.rows.iter().enumerate() {
            for (sx,ch) in row.chars().enumerate() {
                if let Some(mut index) = assets::palette_index(ch) {
                    if let Some((from,to)) = swap { if index == from { index = to; } }
                    let dx = if flip_x { width.saturating_sub(1).saturating_sub(sx) } else { sx };
                    let px=x+dx as i32; let py=y+sy as i32;
                    if (0..W as i32).contains(&px) && (0..H as i32).contains(&py) {
                        self.pixels[py as usize*W+px as usize]=assets::PALETTE[index];
                    }
                }
            }
        }
    }
    pub fn blit_glyph(&mut self, ch: char, x: i32, y: i32, color: usize) {
        let upper=ch.to_ascii_uppercase();
        if let Some(rows)=FONT.get_or_init(assets::glyphs).get(&upper) {
            for (gy,row) in rows.iter().enumerate() { for (gx,p) in row.chars().enumerate() {
                if p=='1' { let px=x+gx as i32; let py=y+gy as i32;
                    if (0..W as i32).contains(&px) && (0..H as i32).contains(&py) { self.pixels[py as usize*W+px as usize]=assets::PALETTE[color.min(15)]; }
                }
            }}
        }
    }
    pub fn text(&mut self, text: &str, x:i32, y:i32, color:usize) {
        let mut cursor=x;
        for ch in text.chars() { if ch==' ' {cursor+=4;} else { self.blit_glyph(ch,cursor,y,color); cursor+=4; } }
    }
    pub fn panel(&mut self,x:i32,y:i32,w:usize,h:usize) {
        let mut py=0; while py<h { let mut px=0; while px<w {
            self.blit(&assets::WALL,x+px as i32,y+py as i32,false,None); px+=8;
        } py+=8; }
    }
    pub fn tile(&mut self,sprite:&Sprite,x:i32,y:i32){self.blit(sprite,x,y,false,None);}
    pub fn frame_hash(&self) -> u64 { self.pixels.iter().fold(1469598103934665603u64,|h,p|(h^u64::from(*p)).wrapping_mul(1099511628211)) }
}

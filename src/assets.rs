//! Hand-authored, palette-indexed pixel art. `.` is transparent; all other glyphs map to PALETTE.
use std::collections::BTreeMap;

pub const W: usize = 320;
pub const H: usize = 180;
/// One fixed 16-color, hue-shifted palette. No pure black or white.
pub const PALETTE: [u32; 16] = [
    0x171923, 0x292b3d, 0x41465b, 0x686b78, // ink / stone
    0x9a897e, 0xd1b99b, 0x375b58, 0x4b8068, // earth / moss
    0x79a878, 0xb6c77c, 0x6b3f55, 0xa4545b, // leaf / ember
    0xd77b61, 0xe9ad71, 0xf2d18a, 0x8bb9bb, // light / sky
];
pub const GLYPHS: [char; 16] = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];

#[derive(Clone, Copy)]
pub struct Sprite { pub name: &'static str, pub rows: &'static [&'static str] }
impl Sprite {
    pub const fn new(name: &'static str, rows: &'static [&'static str]) -> Self { Self { name, rows } }
    pub fn width(&self) -> usize { self.rows.first().map_or(0, |r| r.len()) }
    pub fn height(&self) -> usize { self.rows.len() }
}

pub static HERO_A: Sprite = Sprite::new("hero_idle_a", &[
    "....1111....", "...122221...", "...122d21...", "....1111....",
    "...677776...", "..67777776..", "..677a7776..", "...677776...",
    "....6666....", "...666666...", "...66..66...", "..666..666..",
]);
pub static HERO_B: Sprite = Sprite::new("hero_idle_b", &[
    "....1111....", "...122221...", "...122d21...", "....1111....",
    "...677776...", "..67777776..", "..677a7776..", "...677776...",
    "....6666....", "...666666...", "...6666.....", "..666..66...",
]);
pub static WISP_A: Sprite = Sprite::new("wisp_a", &[
    "...bb...", ".bccccb.", "bccddccb", "bcdeedcb", ".bccccb.", "..b..b..",
]);
pub static WISP_B: Sprite = Sprite::new("wisp_b", &[
    "..b..b..", ".bccccb.", "bccddccb", "bcdeedcb", ".bccccb.", "...bb...",
]);
pub static MOSS_A: Sprite = Sprite::new("mossling_a", &[
    "..7777..", ".788887.", "78888887", "786a6887", ".788887.", "..7..7..",
]);
pub static MOSS_B: Sprite = Sprite::new("mossling_b", &[
    "..7777..", ".788887.", "78888887", "786a6887", ".788887.", ".7....7.",
]);
pub static STALKER_A: Sprite = Sprite::new("stalker_a", &[
    "..66..66..", ".67777776.", "6777777776", "676a777a76", ".67777776.", "..666666..", "..66..66..",
]);
pub static STALKER_B: Sprite = Sprite::new("stalker_b", &[
    ".66....66.", ".67777776.", "6777777776", "676a777a76", ".67777776.", "..666666..", ".66....66.",
]);
pub static BRUTE_A: Sprite = Sprite::new("brute_a", &[
    "...bbbb...", ".bbccccbb.", "bbccccccbb", "bccbbccbcc", "bccbbbbbcc", "bccbbbbbcc", ".bbccccbb.", "..bbbbbb..", ".bb....bb.",
]);
pub static BRUTE_B: Sprite = Sprite::new("brute_b", &[
    "...bbbb...", ".bbccccbb.", "bbccccccbb", "bccbbccbcc", "bccbbbbbcc", "bccbbbbbcc", ".bbccccbb.", "..bbbbbb..", "..bb..bb..",
]);
// Spaces are transparent just like dots; preserving rows as authored makes the silhouette easy to inspect.
pub static BAT_A: Sprite = Sprite::new("bat_a", &[
    "b..b..b", "bbbcbbb", ".bccccb", "..bcb..", "...b...",
]);
pub static BAT_B: Sprite = Sprite::new("bat_b", &[
    ".......", "b..b..b", "bbbcbbb", ".bccccb", "..bcb..",
]);
pub static BOSS_A: Sprite = Sprite::new("lantern_warden_a", &[
    ".....bbbbbb.....", "...bbccccccbb...", "..bccddddddccb..", ".bccdeeeeeedccb.", "bccdeeeeeeeedccb", "bccdebbccbbbedcc", "bccdebccbccbedcc", ".bccdeeeeeedccb.", "..bccddddddccb..", "...bbccccccbb...", "....bbccccbb....", "...bb..bb..bb...", "..bb...bb...bb..",
]);
pub static BOSS_B: Sprite = Sprite::new("lantern_warden_b", &[
    ".....bbbbbb.....", "...bbccccccbb...", "..bccddddddccb..", ".bccdeeeeeedccb.", "bccdeeeeeeeedccb", "bccdebbccbb.edcc", "bccdebccbccbedcc", ".bccdeeeeeedccb.", "..bccddddddccb..", "...bbccccccbb...", ".....bbbb.......", "....bb..bb..bb..", "...bb...bb...bb.",
]);
pub static LANTERN: Sprite = Sprite::new("lantern", &[
    "..bbbb..", ".bccccb.", "bccddccb", "bccdeccb", ".bccccb.", "..b..b..",
]);
pub static HEART: Sprite = Sprite::new("heart", &[".bb.bb.", "bbbbbbb", "bbbbbbb", ".bbbbb.", "..bbb..", "...b..."]);
pub static GEM: Sprite = Sprite::new("ember_shard", &["...d...", "..dcd..", ".dcfcd.", "..dcd..", "...d..."]);
pub static TREE: Sprite = Sprite::new("pine", &[
    ".....77.....", "....7887....", "...788887...", "..78888887..", ".7888888887.", "...777777...", "....6666....", "....6666....", "...666666...",
]);
pub static RUIN: Sprite = Sprite::new("ruin", &[
    "..22222222..", ".2444444442.", "244444444442", "243..44..342", "243..44..342", "244444444442", ".2222222222.", "..222..222..",
]);
// Explicitly authored 8x8 ground tiles, deliberately varied in pattern and value.
pub static FLOOR_A: Sprite = Sprite::new("floor_a", &["44444444", "44444444", "44444444", "44434444", "44444444", "44444444", "44444444", "44444454"]);
pub static FLOOR_B: Sprite = Sprite::new("floor_b", &["44444444", "44445444", "44444444", "44344444", "44444444", "44444444", "44444444", "44444444"]);
pub static FLOOR_C: Sprite = Sprite::new("floor_c", &["44444444", "44444444", "44444444", "44444444", "44444444", "44443444", "44444444", "45444444"]);
pub static WALL: Sprite = Sprite::new("wall_tile", &["22222222", "23333332", "23323332", "23333332", "22222222", "23333332", "23333332", "22222222"]);
pub static STAR: Sprite = Sprite::new("star", &[".f.", "fff", ".f."]);

pub fn palette_index(ch: char) -> Option<usize> {
    if ch == '.' || ch == ' ' { None } else { GLYPHS.iter().position(|c| *c == ch) }
}
pub fn sprites() -> Vec<&'static Sprite> {
    vec![&HERO_A,&HERO_B,&WISP_A,&WISP_B,&MOSS_A,&MOSS_B,&STALKER_A,&STALKER_B,&BRUTE_A,&BRUTE_B,&BAT_A,&BAT_B,&BOSS_A,&BOSS_B,&LANTERN,&HEART,&GEM,&TREE,&RUIN,&FLOOR_A,&FLOOR_B,&FLOOR_C,&WALL,&STAR]
}
pub fn glyphs() -> BTreeMap<char, [&'static str; 7]> {
    // 3x5 hand-authored bitmap font expanded to seven rows with a blank leading/trailing row.
    let raw: [(char, [&str;5]); 37] = [
        ('A',["010","101","111","101","101"]),('B',["110","101","110","101","110"]),('C',["011","100","100","100","011"]),('D',["110","101","101","101","110"]),('E',["111","100","110","100","111"]),('F',["111","100","110","100","100"]),('G',["011","100","101","101","011"]),('H',["101","101","111","101","101"]),('I',["111","010","010","010","111"]),('J',["001","001","001","101","010"]),('K',["101","101","110","101","101"]),('L',["100","100","100","100","111"]),('M',["101","111","111","101","101"]),('N',["101","111","111","111","101"]),('O',["010","101","101","101","010"]),('P',["110","101","110","100","100"]),('Q',["010","101","101","111","011"]),('R',["110","101","110","101","101"]),('S',["011","100","010","001","110"]),('T',["111","010","010","010","010"]),('U',["101","101","101","101","111"]),('V',["101","101","101","101","010"]),('W',["101","101","111","111","101"]),('X',["101","101","010","101","101"]),('Y',["101","101","010","010","010"]),('Z',["111","001","010","100","111"]),
        ('0',["111","101","101","101","111"]),('1',["010","110","010","010","111"]),('2',["110","001","010","100","111"]),('3',["110","001","010","001","110"]),('4',["101","101","111","001","001"]),('5',["111","100","110","001","110"]),('6',["011","100","110","101","010"]),('7',["111","001","010","010","010"]),('8',["010","101","010","101","010"]),('9',["010","101","011","001","110"]),('-', ["000","000","111","000","000"]),
    ];
    raw.into_iter().map(|(c, rows)| { let mut out=["000";7]; for (i,row) in rows.iter().enumerate(){ out[i+1]=*row; } (c,out) }).collect()
}

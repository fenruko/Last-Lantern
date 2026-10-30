//! Pixel-font menus and HUD. Panels are assembled only from authored tile sprites.
use crate::{assets,game::Game,render::Renderer};
pub fn title(r:&mut Renderer,g:&Game){
 r.panel(51,20,218,137);r.blit(&assets::LANTERN,146,34,false,None);r.blit(&assets::STAR,88,33,false,None);r.blit(&assets::STAR,226,54,false,None);
 r.text("LAST LANTERN",112,62,14);r.text("THE HOLLOW WAITS",113,76,5);
 for (i,label) in ["BEGIN JOURNEY","SETTINGS","CONTINUE"].iter().enumerate(){let y=99+i as i32*14;if i==g.menu_index{r.blit(&assets::GEM,81,y-2,false,None);}r.text(label,94,y,if i==g.menu_index{14}else{5});}
 r.text("ARROWS MOVE  Z STRIKE",83,139,3);r.text("P PAUSE  M SAVE AND EXIT",79,148,3);
}
pub fn hud(r:&mut Renderer,g:&Game){
 r.blit(&assets::LANTERN,5,4,false,None);for i in 0..g.max_hp{r.blit(&assets::HEART,19+i*10,5,false,if i<g.hp{None}else{Some((11,2))});}
 r.panel(250,3,66,17);r.text("AREA",257,7,5);r.text(match g.level{0=>"01",1=>"02",_=>"03"},293,7,14);
 r.text(&format!("SHARDS {:02}   SCORE {:04}",g.shards,g.score),7,166,5);
 if g.saved&&g.frame%100<40{r.text("SAVED",278,28,8);}
}
pub fn pause(r:&mut Renderer){r.panel(93,44,134,94);r.text("PAUSED",137,58,14);r.text("THE DARK CAN WAIT",119,73,5);r.text("ENTER SAVE AND RETURN",105,94,5);r.text("ESC RESUME",132,109,8);}
pub fn settings(r:&mut Renderer,difficulty:f32){r.panel(66,38,188,104);r.text("SETTINGS",126,53,14);r.text("DIFFICULTY",93,76,5);r.text(&format!("{:02}", (difficulty*100.0) as u32),204,76,14);r.text("LEFT RIGHT TO ADJUST",95,94,5);r.text("ENTER OR ESC BACK",101,113,8);r.text("GAMEPAD JS0 SUPPORTED",96,129,3);}
pub fn end_screen(r:&mut Renderer,won:bool,score:u32){r.panel(80,43,160,96);if won{r.blit(&assets::LANTERN,155,52,false,None);r.text("DAWN RETURNS",119,75,14);r.text("THE LAST LIGHT ENDURES",97,90,5);}else{r.text("LIGHTS OUT",130,63,12);r.text("THE HOLLOW CLAIMS YOU",99,82,5);}r.text(&format!("FINAL SCORE {:04}",score),115,103,8);r.text("ENTER TO RETURN",118,121,5);}

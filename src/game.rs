//! Game state, combat, progression, persistence, and Linux joystick input.
use crate::{ai::{self,Brain,Node,Role}, assets, render::Renderer, ui};
use minifb::Key;
use std::{fs,io::Read,path::PathBuf};

#[derive(Clone,Copy,PartialEq,Eq)] pub enum Screen { Title, Playing, Paused, Settings, Defeat, Victory }
#[derive(Clone,Copy,PartialEq,Eq)] pub enum EnemyKind { Wisp, Mossling, Stalker, Brute, Warden }
#[derive(Clone)] pub struct Enemy {pub kind:EnemyKind,pub x:f32,pub y:f32,pub hp:i32,pub brain:Brain,pub anim:u8,pub cooldown:u8}
#[derive(Clone,Copy)] pub struct Particle {pub x:i32,pub y:i32,pub life:u8,pub color:usize}
#[derive(Default)] pub struct Input {pub left:bool,pub right:bool,pub up:bool,pub down:bool,pub attack:bool,pub confirm:bool,pub pause:bool,pub back:bool,pub menu:bool}
impl Input {pub fn keys(keys:&[Key])->Self{let has=|k|keys.contains(&k);Self{left:has(Key::Left)||has(Key::A),right:has(Key::Right)||has(Key::D),up:has(Key::Up)||has(Key::W),down:has(Key::Down)||has(Key::S),attack:has(Key::Space)||has(Key::Z),confirm:has(Key::Enter)||has(Key::Space),pause:has(Key::Escape)||has(Key::P),back:has(Key::Escape),menu:has(Key::M)}}}

#[derive(Default)] pub struct Gamepad {pub x:i16,pub y:i16,pub buttons:u32}
pub struct Joystick {file:Option<fs::File>,pub state:Gamepad}
impl Joystick {
 pub fn open()->Self {use std::os::unix::fs::OpenOptionsExt;let file=fs::OpenOptions::new().read(true).custom_flags(libc::O_NONBLOCK).open("/dev/input/js0").ok();Self{file,state:Gamepad::default()}}
 pub fn poll(&mut self){if let Some(file)=self.file.as_mut(){let mut buf=[0u8;8];while let Ok(8)=file.read(&mut buf){let value=i16::from_ne_bytes([buf[4],buf[5]]);let kind=buf[6]&0x7f;let number=buf[7];if kind==2{if number==0{self.state.x=value;}else if number==1{self.state.y=value;}}else if kind==1 && number<32{if value!=0{self.state.buttons|=1<<number;}else{self.state.buttons&=!(1<<number);}}}}}
 pub fn apply(&self,input:&mut Input){input.left|=self.state.x < -12000;input.right|=self.state.x>12000;input.up|=self.state.y < -12000;input.down|=self.state.y>12000;input.attack|=self.state.buttons&1!=0;input.confirm|=self.state.buttons&2!=0;input.pause|=self.state.buttons&8!=0;}
}

pub struct Game {pub screen:Screen,pub x:f32,pub y:f32,pub hp:i32,pub max_hp:i32,pub shards:u32,pub level:u8,pub score:u32,pub enemies:Vec<Enemy>,pub particles:Vec<Particle>,pub frame:u64,pub attack_cd:u8,pub invulnerable:u8,pub hit_stop:u8,pub shake:u8,pub menu_index:usize,pub difficulty:f32,pub saved:bool,pub player_facing:i8}
impl Game {
 pub fn new()->Self{let mut g=Self{screen:Screen::Title,x:60.0,y:120.0,hp:5,max_hp:5,shards:0,level:0,score:0,enemies:vec![],particles:vec![],frame:0,attack_cd:0,invulnerable:0,hit_stop:0,shake:0,menu_index:0,difficulty:1.0,saved:false,player_facing:1};g.spawn_area();g}
 fn spawn_area(&mut self){self.enemies.clear();let configs:&[(EnemyKind,f32,f32,i32,Role,u8)]=match self.level{0=>&[(EnemyKind::Wisp,245.,108.,2,Role::Hunter,7),(EnemyKind::Mossling,340.,132.,3,Role::Flanker,9),(EnemyKind::Wisp,440.,92.,2,Role::Hunter,6)],1=>&[(EnemyKind::Stalker,250.,110.,4,Role::Flanker,8),(EnemyKind::Brute,370.,130.,6,Role::Guard,13),(EnemyKind::Mossling,465.,100.,3,Role::Hunter,8)],_=>&[(EnemyKind::Warden,410.,116.,20,Role::Hunter,6),(EnemyKind::Stalker,250.,95.,4,Role::Flanker,6),(EnemyKind::Wisp,500.,90.,3,Role::Hunter,5)]};
  for (kind,x,y,hp,role,reaction) in configs {let boost=((self.difficulty-1.0)*4.0).round() as i32;self.enemies.push(Enemy{kind:*kind,x:*x,y:*y,hp:*hp+(boost as i32),brain:Brain::new(((*x/8.) as i32,(*y/8.) as i32),*role,*reaction),anim:0,cooldown:0});}}
 pub fn continue_game(&mut self){if let Some((level,hp,max_hp,shards,score))=load_save(){self.level=level.min(2);self.max_hp=max_hp.clamp(5,7);self.hp=hp.clamp(1,self.max_hp);self.shards=shards;self.score=score;self.x=60.;}else{self.level=0;self.hp=5;self.max_hp=5;self.shards=0;self.score=0;self.x=60.;}self.y=120.;self.screen=Screen::Playing;self.spawn_area();}
 pub fn save(&mut self){let _=fs::write(save_path(),format!("{} {} {} {} {}",self.level,self.hp,self.max_hp,self.shards,self.score));self.saved=true;}
 pub fn update(&mut self,input:Input){self.frame=self.frame.wrapping_add(1);if self.hit_stop>0{self.hit_stop-=1;return;}if self.invulnerable>0{self.invulnerable-=1;}if self.attack_cd>0{self.attack_cd-=1;}if self.shake>0{self.shake-=1;}for p in &mut self.particles{p.life=p.life.saturating_sub(1);}self.particles.retain(|p|p.life>0);
  match self.screen {
   Screen::Title=>{if input.down{self.menu_index=(self.menu_index+1)%3;}if input.up{self.menu_index=(self.menu_index+2)%3;}if input.confirm{if self.menu_index==0{self.continue_game();}else if self.menu_index==1{self.screen=Screen::Settings;}else{self.menu_index=0;}}},
   Screen::Settings=>{if input.left{self.difficulty=(self.difficulty-0.25).max(0.75);}if input.right{self.difficulty=(self.difficulty+0.25).min(1.75);}if input.back||input.confirm{self.screen=Screen::Title;}},
   Screen::Paused=>{if input.pause||input.back{self.screen=Screen::Playing;}else if input.confirm{self.save();self.screen=Screen::Title;}},
   Screen::Playing=>self.update_play(input),
   Screen::Defeat|Screen::Victory=>{if input.confirm||input.back{self.screen=Screen::Title;self.menu_index=0;}},
  }
 }
 fn update_play(&mut self,input:Input){if input.pause{self.screen=Screen::Paused;self.save();return;}if input.menu{self.save();self.screen=Screen::Title;return;}
  let speed=1.65f32;let mut dx=0.;let mut dy=0.;if input.left{dx-=speed;self.player_facing=-1;}if input.right{dx+=speed;self.player_facing=1;}if input.up{dy-=speed;}if input.down{dy+=speed;}self.x=(self.x+dx).clamp(10.,945.);self.y=(self.y+dy).clamp(54.,148.);
  if input.attack&&self.attack_cd==0{self.attack_cd=18;self.hit_stop=2;let mut hit=false;for enemy in &mut self.enemies{let dist=(enemy.x-self.x).abs()+(enemy.y-self.y).abs();let in_front=(enemy.x-self.x)*self.player_facing as f32>=-4.;if dist<35.&&in_front{enemy.hp-=2;hit=true;for n in 0..7{let c=if n%2==0{13}else{14};self.particles.push(Particle{x:enemy.x as i32+(n as i32%4)*3,y:enemy.y as i32-(n as i32/4)*3,life:9+(n as u8%5),color:c});}}}if hit{self.shake=3;}}
  let player_node=((self.x/8.) as i32,(self.y/8.) as i32);let bounds=(120,22);let cover:&[Node]=match self.level{0=>&[(22,13),(23,13),(51,12)],1=>&[(29,14),(30,14),(58,12)],_=>&[(41,14),(42,14),(71,12)]};
  for i in 0..self.enemies.len(){let enemy=&mut self.enemies[i];enemy.anim=enemy.anim.wrapping_add(1);if enemy.cooldown>0{enemy.cooldown-=1;}let node=((enemy.x/8.) as i32,(enemy.y/8.) as i32);let visible=ai::line_of_sight(node,player_node,cover);if self.frame%8==0{enemy.brain.pos=node;if !visible && (enemy.x-self.x).abs()+(enemy.y-self.y).abs()<88. {enemy.brain.hear(player_node);}let group=if enemy.brain.role==Role::Hunter{Some(player_node)}else{None};if let Some(next)=enemy.brain.tick(player_node,visible,bounds,&[],self.difficulty,group){enemy.x=next.0 as f32*8.;enemy.y=next.1 as f32*8.;}}
   let distance=(enemy.x-self.x).abs()+(enemy.y-self.y).abs();if distance<18.&&enemy.cooldown==0&&self.invulnerable==0{enemy.cooldown=40;self.hp-=if enemy.kind==EnemyKind::Warden{2}else{1};self.invulnerable=55;self.shake=6;self.hit_stop=3;for n in 0..9{self.particles.push(Particle{x:self.x as i32+(n as i32%3)*3,y:self.y as i32-(n as i32/3)*3,life:12,color:11});}if self.hp<=0{self.hp=0;self.screen=Screen::Defeat;}}
  }
  let before=self.enemies.len();self.enemies.retain(|e|e.hp>0);let killed=before-self.enemies.len();if killed>0{self.score+=killed as u32*100;self.shards+=killed as u32;}
  if self.shards>=3&&self.max_hp<7{self.shards-=3;self.max_hp+=1;self.hp=(self.hp+1).min(self.max_hp);}
  if self.enemies.is_empty(){if self.level<2{self.level+=1;self.x=22.;self.y=120.;self.spawn_area();self.save();}else{self.screen=Screen::Victory;self.save();}}
 }
 pub fn render(&self,r:&mut Renderer){let shake=if self.shake>0&&self.frame%2==0{(self.shake as i32%3)-1}else{0};let camera=((self.x as i32-150)+shake).clamp(0,660);r.clear_with(0);
  let shift=match self.level{0=>Some((6,15)),1=>Some((6,2)),_=>Some((6,10))};
  for ty in 0..23{for tx in 0..42{let variant=((tx*13+ty*7+self.level as i32*19)%3).abs();let tile=match variant{0=>&assets::FLOOR_A,1=>&assets::FLOOR_B,_=>&assets::FLOOR_C};let swap=match self.level{0=>Some((4,6)),1=>Some((4,2)),_=>Some((4,10))};r.blit(tile,tx*8,ty*8,false,swap);if ty==0||ty==22{r.blit(&assets::WALL,tx*8,ty*8,false,shift);}}}
  // Water/mist horizon is a hand-authored sprite layer repeated with parallax floor tiles.
  for tx in 0..42{if (tx+self.level as i32)%4==0{r.blit(&assets::FLOOR_B,tx*8,42,false,Some((4,15)));}}
  // Parallax night sky and silhouettes are drawn over the tiled ground as authored sprites.
  for n in 0..16{let sx=((n*47+13-(camera/4))%350) as i32-10;let sy=(n*31+9)%53;r.blit(&assets::STAR,sx,sy as i32,false,shift);}
  for n in 0..8{let wx=n*137+42;let sx=wx-camera/2;if sx>-20&&sx<320{r.blit(&assets::TREE,sx,75+(n%3) as i32*4,false,shift);}}
  if self.level>0{for n in 0..4{let sx=n*185+95-camera/2;if sx>-30&&sx<320{r.blit(&assets::RUIN,sx,100+(n%2) as i32*8,false,shift);}}}
  let mut entities:Vec<(f32,bool,usize)>=self.enemies.iter().enumerate().map(|(i,e)|(e.y,false,i)).collect();entities.sort_by(|a,b|a.0.total_cmp(&b.0));
  for (_,_,idx) in entities{let e=&self.enemies[idx];let sx=e.x as i32-camera;let sy=e.y as i32;let (a,b)=match e.kind{EnemyKind::Wisp=>(&assets::WISP_A,&assets::WISP_B),EnemyKind::Mossling=>(&assets::MOSS_A,&assets::MOSS_B),EnemyKind::Stalker=>(&assets::STALKER_A,&assets::STALKER_B),EnemyKind::Brute=>(&assets::BRUTE_A,&assets::BRUTE_B),EnemyKind::Warden=>(&assets::BOSS_A,&assets::BOSS_B)};let s=if e.anim/12%2==0{a}else{b};let swap=if e.kind==EnemyKind::Warden{Some((13,14))}else{None};r.blit(s,sx-(s.width()/2) as i32,sy-(s.height() as i32),false,swap);if e.kind==EnemyKind::Warden{r.blit(&assets::LANTERN,sx-4,sy-26,false,None);}}
  let ps=self.x as i32-camera;let hero=if self.frame/20%2==0{&assets::HERO_A}else{&assets::HERO_B};if self.invulnerable==0||self.frame%6<3{r.blit(hero,ps-6,self.y as i32-12,self.player_facing<0,None);}
  for p in &self.particles{r.blit(&assets::GEM,p.x-camera,p.y,false,Some((13,p.color)));}
  if self.screen==Screen::Title{ui::title(r,self);}else if self.screen==Screen::Paused{ui::pause(r);}else if self.screen==Screen::Settings{ui::settings(r,self.difficulty);}else if self.screen==Screen::Defeat{ui::end_screen(r,false,self.score);}else if self.screen==Screen::Victory{ui::end_screen(r,true,self.score);}else{ui::hud(r,self);}
 }
 pub fn track(&self)->usize{match self.screen{Screen::Title|Screen::Settings=>0,Screen::Victory|Screen::Defeat=>0,Screen::Paused=>1,Screen::Playing if self.level==2=>2,_=>1}}
}
fn save_path()->PathBuf{PathBuf::from("last-lantern.sav")}
fn load_save()->Option<(u8,i32,i32,u32,u32)>{let text=fs::read_to_string(save_path()).ok()?;let mut p=text.split_whitespace();Some((p.next()?.parse().ok()?,p.next()?.parse().ok()?,p.next()?.parse().ok()?,p.next()?.parse().ok()?,p.next()?.parse().ok()?))}

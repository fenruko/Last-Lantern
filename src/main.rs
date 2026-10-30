use last_lantern::{ai::{Brain,Role},assets,game::{Enemy,EnemyKind,Game,Input,Joystick},render::Renderer};
use minifb::{Scale,Window,WindowOptions};
use std::{error::Error,fs::{self,File},path::{Path,PathBuf},time::Instant};

fn main()->Result<(),Box<dyn Error>>{
 let args:Vec<String>=std::env::args().collect();let has=|flag:&str|args.iter().any(|a|a==flag);
 if has("--help")||has("-h"){println!("Last Lantern\n  last-lantern [--no-audio]\n  last-lantern --headless --frames N --dump out/\n  last-lantern --headless --art-report out/\n  last-lantern --benchmark");return Ok(());}
 if has("--headless") {let frames=arg_value(&args,"--frames","1").parse::<usize>()?;let dir=PathBuf::from(arg_value(&args,"--dump","out/"));fs::create_dir_all(&dir)?;let mut game=Game::new();game.continue_game();let mut renderer=Renderer::new();for n in 0..frames{game.frame=n as u64;game.render(&mut renderer);write_png(dir.join(format!("frame-{n:04}.png")),assets::W as u32,assets::H as u32,&renderer.pixels)?;}
  write_contact_and_report(&dir)?;println!("Headless render complete: {frames} frame(s) in {}",dir.display());return Ok(());}
 if has("--art-report"){let dir=PathBuf::from(arg_value(&args,"--art-report","out/"));fs::create_dir_all(&dir)?;write_contact_and_report(&dir)?;println!("Contact sheet and art report written to {}",dir.display());return Ok(());}
 if has("--benchmark"){let mut g=Game::new();g.continue_game();let mut r=Renderer::new();let mut samples=Vec::with_capacity(1000);for _ in 0..1000{let t=Instant::now();g.update(Default::default());g.render(&mut r);samples.push(t.elapsed());}samples.sort_unstable();let p99=samples[989].as_secs_f64()*1000.0;println!("CPU software renderer p99: {p99:.3} ms (target < 16 ms)");if p99>=16.0{return Err("performance target exceeded".into());}return Ok(());}
 run_window(!has("--no-audio"))
}
fn arg_value<'a>(args:&'a [String],flag:&str,default:&'a str)->&'a str{args.windows(2).find(|w|w[0]==flag).map(|w|w[1].as_str()).unwrap_or(default)}
fn run_window(audio_enabled:bool)->Result<(),Box<dyn Error>>{
 let mut window=Window::new("Last Lantern — The Hollow Waits",assets::W as usize,assets::H as usize,WindowOptions{scale:Scale::X4,..WindowOptions::default()})?;
 #[allow(deprecated)]
 window.limit_update_rate(Some(std::time::Duration::from_micros(16_667)));
 let mut game=Game::new();let mut renderer=Renderer::new();let mut joystick=Joystick::open();let mut audio=if audio_enabled{last_lantern::audio::Audio::new()}else{None};let mut track=usize::MAX;let mut prior_score=0;let mut prior_hp=game.hp;
 while window.is_open(){let keys=window.get_keys();let mut input=Input::keys(&keys);joystick.poll();joystick.apply(&mut input);game.update(input);game.render(&mut renderer);window.update_with_buffer(&renderer.pixels,assets::W as usize,assets::H as usize)?;
  if let Some(sound)=audio.as_ref(){let current=game.track();if current!=track{sound.set_track(current);track=current;}if game.score>prior_score{sound.sfx(0);}if game.hp<prior_hp{sound.sfx(1);}}
  prior_score=game.score;prior_hp=game.hp;
 }
 Ok(())
}
fn write_png(path:impl AsRef<Path>,width:u32,height:u32,pixels:&[u32])->Result<(),Box<dyn Error>>{
 let mut file=File::create(path)?;let mut encoder=png::Encoder::new(&mut file,width,height);encoder.set_color(png::ColorType::Rgb);encoder.set_depth(png::BitDepth::Eight);let mut writer=encoder.write_header()?;let mut rgb=Vec::with_capacity(pixels.len()*3);for p in pixels{rgb.push((p>>16) as u8);rgb.push((p>>8) as u8);rgb.push(*p as u8);}writer.write_image_data(&rgb)?;Ok(())
}
fn write_contact_and_report(dir:&Path)->Result<(),Box<dyn Error>>{
 let mut renderer=Renderer::new();let mut shots:Vec<Vec<u32>>=Vec::new();let title=Game::new();title.render(&mut renderer);shots.push(renderer.pixels.clone());
 let mut game=Game::new();game.continue_game();game.render(&mut renderer);shots.push(renderer.pixels.clone());
 game.level=1;game.x=410.;game.render(&mut renderer);shots.push(renderer.pixels.clone());
 game.level=2;game.x=410.;game.enemies.clear();game.enemies.push(Enemy{kind:EnemyKind::Warden,x:205.,y:118.,hp:20,brain:Brain::new((25,14),Role::Hunter,0),anim:19,cooldown:0});game.render(&mut renderer);shots.push(renderer.pixels.clone());
 let cw=assets::W*2;let ch=assets::H*2;let mut sheet=vec![assets::PALETTE[0];cw*ch];for (tile,shot) in shots.iter().enumerate(){let ox=(tile%2)*assets::W;let oy=(tile/2)*assets::H;for y in 0..assets::H{for x in 0..assets::W{let p=shot[y*assets::W+x];let i=(oy+y*2)*cw+ox+x*2;sheet[i]=p;sheet[i+1]=p;sheet[i+cw]=p;sheet[i+cw+1]=p;}}}
 write_png(dir.join("contact-sheet.png"),cw as u32,ch as u32,&sheet)?;
 let mut report=String::from("LAST LANTERN — ART INSPECTION REPORT\nHand-authored palette-indexed sprite inventory and rendered key screens.\n\nSPRITE SIZES (pixels)\n");for s in assets::sprites(){report.push_str(&format!("{:20} {:2} x {:2}\n",s.name,s.width(),s.height()));}
 let mut hist=[0u64;16];for sprite in assets::sprites(){for row in sprite.rows{for ch in row.chars(){if let Some(index)=assets::palette_index(ch){hist[index]+=1;}}}}
 report.push_str("\nPALETTE HISTOGRAM (authored sprite pixels)\n");for (i,count) in hist.iter().enumerate(){report.push_str(&format!("{}  #{:06x}  {}\n",assets::GLYPHS[i],assets::PALETTE[i],count));}
 report.push_str("\nASCII DOWNSAMPLES — title / area one / area two / boss\nSymbols 0-f are fixed palette indices sampled every 4x4 pixels; dots are transparent/background dark.\n");for (scene,shot) in ["TITLE","AREA ONE","AREA TWO","WARDEN BOSS"].iter().zip(shots.iter()) {report.push_str(&format!("\n[{scene}]\n"));for y in (0..assets::H).step_by(4){for x in (0..assets::W).step_by(4){let color=shot[y*assets::W+x];let idx=assets::PALETTE.iter().position(|c|*c==color).unwrap_or(0);report.push(assets::GLYPHS[idx]);}report.push('\n');}}
 report.push_str("\nLINTER NOTES\n- Runtime frame pixels are copied only from the fixed palette and authored sprite/font grids.\n- Nearest-neighbor integer scaling is used for the contact sheet and window.\n- Run `cargo test` for palette, grid, outline, flat-region, primitive, AI, and render timing checks.\n");fs::write(dir.join("art-report.txt"),report)?;Ok(())
}

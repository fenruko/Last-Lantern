//! Last Lantern: a CPU-rendered, palette-indexed action-adventure.
pub mod ai;
pub mod assets;
pub mod audio;
pub mod game;
pub mod render;
pub mod ui;

#[cfg(test)] mod verification {
 use super::{assets,game::Game,render::Renderer};
 use std::{collections::VecDeque,time::Instant};
 #[test] fn art_linter_palette_grid_flat_regions_and_outlines(){
  assert!(assets::PALETTE.len()<=32,"palette exceeds 32 colors");
  assert!(!assets::PALETTE.contains(&0x000000)&&!assets::PALETTE.contains(&0xffffff),"pure black/white forbidden");
  for s in assets::sprites(){
   let w=s.width();assert!(w>0,"empty sprite {}",s.name);
   assert!(s.rows.iter().all(|r|r.chars().count()==w),"ragged pixel grid in {}",s.name);
   let h=s.height();let mut grid=vec![None;w*h];
   for (y,row) in s.rows.iter().enumerate(){for (x,ch) in row.chars().enumerate(){let ix=assets::palette_index(ch);assert!(ch=='.'||ch==' '||ix.is_some(),"out-of-palette pixel in {}",s.name);grid[y*w+x]=ix;}}
   // Connected solid-color regions are bounded; no anti-alias/blended pixel can occur because
   // every pixel is a palette index rather than an RGB interpolation.
   let mut seen=vec![false;grid.len()];for i in 0..grid.len(){if seen[i]||grid[i].is_none(){continue;}let color=grid[i];let mut q=VecDeque::new();q.push_back(i);seen[i]=true;let mut count=0;while let Some(k)=q.pop_front(){count+=1;let x=k%w;let y=k/w;for n in [if x>0{Some(k-1)}else{None},if x+1<w{Some(k+1)}else{None},if y>0{Some(k-w)}else{None},if y+1<h{Some(k+w)}else{None}].into_iter().flatten(){if !seen[n]&&grid[n]==color{seen[n]=true;q.push_back(n);}}}assert!(count<=64,"flat color region of {count} pixels in {}",s.name);}
   // Dark outlines must not form a long, perfectly straight bar.
   for row in s.rows{let mut run=0;let mut last='.';for ch in row.chars(){if matches!(ch,'0'|'1'|'2'){if ch==last{run+=1;}else{run=1;last=ch;}}else{run=0;last='.';}assert!(run<12,"long straight outline in {}",s.name);}}
   for x in 0..w{let mut run=0;let mut last='.';for row in s.rows{let ch=row.chars().nth(x).unwrap();if matches!(ch,'0'|'1'|'2'){if ch==last{run+=1;}else{run=1;last=ch;}}else{run=0;last='.';}assert!(run<12,"long vertical outline in {}",s.name);}}
  }
  let source=include_str!("render.rs");for forbidden in ["draw_rect(","draw_circle(","draw_line(","fill_rect(","fill_circle(","fill_triangle("]{assert!(!source.contains(forbidden),"shape primitive forbidden: {forbidden}");}
 }
 #[test] fn software_renderer_p99_under_sixteen_milliseconds(){
  let mut game=Game::new();game.continue_game();let mut renderer=Renderer::new();let mut times=Vec::with_capacity(120);
  for _ in 0..120{let begin=Instant::now();game.update(Default::default());game.render(&mut renderer);times.push(begin.elapsed());}
  times.sort_unstable();let p99=times[118].as_secs_f64()*1000.0;println!("software render p99: {p99:.3} ms");assert!(p99<16.0,"p99 software frame time {p99:.3}ms exceeds 16ms");
 }
}

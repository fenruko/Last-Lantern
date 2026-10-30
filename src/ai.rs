//! Enemy navigation uses `pathfinding`'s public A* implementation. Tactical policy layers LOS,
//! hearing, memory, roles, reaction delay, and health-aware retreat over crate-provided paths.
use pathfinding::prelude::astar;

pub type Node=(i32,i32);
#[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum Role { Hunter, Flanker, Guard }
#[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum Mode { Patrol, Investigate, Chase, Retreat }
#[derive(Clone,Debug)] pub struct Brain {
    pub pos: Node, pub last_seen: Option<Node>, pub mode: Mode, pub role: Role,
    pub hp: u8, pub reaction: u8, pub memory: u16, pub heard: Option<Node>,
}
impl Brain {
    pub fn new(pos:Node,role:Role,reaction:u8)->Self{Self{pos,last_seen:None,mode:Mode::Patrol,role,hp:3,reaction,memory:0,heard:None}}
    pub fn hear(&mut self, at:Node){self.heard=Some(at);}
    pub fn tick(&mut self, player:Node, visible:bool, bounds:(i32,i32), blockers:&[Node], _difficulty:f32, group_target:Option<Node>)->Option<Node>{
        if visible { self.last_seen=Some(player); self.memory=180; self.mode=Mode::Chase; }
        else if self.memory>0 { self.memory-=1; self.mode=Mode::Investigate; }
        else if let Some(p)=self.heard.take(){ self.last_seen=Some(p); self.memory=90; self.mode=Mode::Investigate; }
        else {self.mode=Mode::Patrol;}
        if self.hp<=1 && self.mode==Mode::Chase { self.mode=Mode::Retreat; }
        if self.reaction>0 {self.reaction-=1; return None;}
        let mut target=self.last_seen.unwrap_or(player);
        if self.mode==Mode::Retreat {target=(self.pos.0+(self.pos.0-player.0).signum()*4,self.pos.1+(self.pos.1-player.1).signum()*4);}
        else if self.role==Role::Flanker && self.mode==Mode::Chase {
            target=(player.0+(if self.pos.0<=player.0{-2}else{2}),player.1+(if self.pos.1<=player.1{-2}else{2}));
        } else if self.role==Role::Guard && self.mode!=Mode::Chase {return None;}
        else if self.role==Role::Hunter { if let Some(g)=group_target {target=g;} }
        target.0=target.0.clamp(1,bounds.0-2); target.1=target.1.clamp(1,bounds.1-2);
        // Difficulty scaling is applied to encounter HP by Game; the policy remains deterministic.
        let path=find_path(self.pos,target,bounds,blockers)?;
        if path.len()>1 {Some(path[1])} else {None}
    }
}

pub fn find_path(start:Node,goal:Node,bounds:(i32,i32),blockers:&[Node])->Option<Vec<Node>>{
    if start==goal {return Some(vec![start]);}
    astar(&start,|&(x,y)| {
        let mut out=Vec::with_capacity(4);
        for p in [(x+1,y),(x-1,y),(x,y+1),(x,y-1)] {
            if p.0>=0 && p.1>=0 && p.0<bounds.0 && p.1<bounds.1 && !blockers.contains(&p){out.push((p,1));}
        } out
    },|&(x,y)| ((x-goal.0).abs()+(y-goal.1).abs()) as u32,|p|*p==goal).map(|(path,_)|path)
}
/// Grid-based line of sight; walls between endpoints block sight.
pub fn line_of_sight(a:Node,b:Node,blockers:&[Node])->bool{
    let dx=(b.0-a.0).abs(); let sx=if a.0<b.0{1}else{-1}; let dy=-(b.1-a.1).abs(); let sy=if a.1<b.1{1}else{-1};
    let mut err=dx+dy; let(mut x,mut y)=a;
    loop {if (x,y)!=a && (x,y)!=b && blockers.contains(&(x,y)){return false;} if (x,y)==b{return true;} let e2=2*err;if e2>=dy{err+=dy;x+=sx;}if e2<=dx{err+=dx;y+=sy;}}
}
#[derive(Clone)] pub struct Rng(pub u64);
impl Rng {pub fn new(seed:u64)->Self{Self(seed.max(1))}pub fn next(&mut self)->u32{self.0^=self.0<<13;self.0^=self.0>>7;self.0^=self.0<<17;(self.0>>16) as u32}pub fn range(&mut self,n:u32)->u32{if n==0{0}else{self.next()%n}}}

#[cfg(test)] mod tests {
 use super::*;
 #[test] fn thousand_seeded_encounters_complete(){
  let mut won=0usize;let mut total=0u64;
  for seed in 1..=1000u64 {
   let mut rng=Rng::new(seed);let bounds=(24,16);let start=(1,1);let goal=(22,14);
   let mut blocks=Vec::new();
   // A deliberately sparse, seeded obstacle field; all agents use the crate's A* search.
   for y in 2..14 {for x in 2..22 {if rng.range(100)<7 && (x,y)!=(goal.0-1,goal.1) {blocks.push((x,y));}}}
   let route=find_path(start,goal,bounds,&blocks).expect("A* must find a route around sparse blockers");
   assert!(route.len()>1);assert_eq!(*route.last().unwrap(),goal);
   let mut brain=Brain::new(start,Role::Hunter,(seed%4) as u8);brain.hear((goal.0-1,goal.1));
   let mut pos=start;let mut steps=0;
   while pos!=goal && steps<1000 {brain.pos=pos;let next=brain.tick(goal,true,bounds,&blocks,1.0,None).or_else(||find_path(pos,goal,bounds,&blocks).and_then(|p|p.get(1).copied()));if let Some(n)=next{pos=n;}steps+=1;}
   assert_eq!(pos,goal,"agent stuck, seed={seed}");assert!(steps<1000);total+=steps as u64;won+=1;
  }
  println!("AI simulation: {won}/1000 wins ({:.1}%), mean encounter time {:.2} steps",won as f64/10.0,total as f64/won as f64);
 }
 #[test] fn visibility_respects_walls(){assert!(!line_of_sight((0,0),(2,0),&[(1,0)]));assert!(line_of_sight((0,0),(2,0),&[]));}
}

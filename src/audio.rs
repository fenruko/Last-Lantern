//! Original code-composed tracker music and synthesized effects. No samples or borrowed melodies.
#[cfg(feature="audio")]
mod live {
 use rodio::{OutputStream, OutputStreamHandle, Sink, Source, buffer::SamplesBuffer};
 use std::{sync::{Arc,atomic::{AtomicUsize,Ordering}},time::Duration};
 const RATE:u32=22050;
 // Three original modal motifs: title (Dorian), exploration (Aeolian), boss (Phrygian).
 const TRACKS:[[u8;16];3]=[
  [50,57,53,62,50,57,65,62,48,55,52,60,48,55,62,60],
  [45,52,48,55,45,52,57,55,41,48,45,53,41,48,55,53],
  [38,39,45,50,38,39,48,45,36,37,43,48,36,37,46,43],
 ];
 pub struct SynthSource { track:Arc<AtomicUsize>, sample:u64 }
 impl Iterator for SynthSource {type Item=f32; fn next(&mut self)->Option<f32>{
  let track=self.track.load(Ordering::Relaxed)%3;let per_step=RATE as u64/3;let step=(self.sample/per_step) as usize%16;let local=(self.sample%per_step) as f32/per_step as f32;let note=TRACKS[track][step];
  let freq=440.0*2.0f32.powf((note as f32-69.0)/12.0);let phase=(self.sample as f32*freq/RATE as f32)*std::f32::consts::TAU;
  let envelope=(1.0-local*0.75).max(0.0);let bass=if step%4==0{(phase*0.5).sin()*0.10}else{0.0};let melody=phase.sin()*0.065*envelope;
  self.sample=self.sample.wrapping_add(1);Some((melody+bass).clamp(-0.7,0.7))
 }}
 impl Source for SynthSource {fn current_frame_len(&self)->Option<usize>{None}fn channels(&self)->u16{1}fn sample_rate(&self)->u32{RATE}fn total_duration(&self)->Option<Duration>{None}}
 pub struct Audio { _stream:OutputStream, handle:OutputStreamHandle, _music:Sink, track:Arc<AtomicUsize> }
 impl Audio {
  pub fn new()->Option<Self>{let(stream,handle)=OutputStream::try_default().ok()?;let music=Sink::try_new(&handle).ok()?;let track=Arc::new(AtomicUsize::new(0));music.append(SynthSource{track:track.clone(),sample:0});music.set_volume(0.28);Some(Self{_stream:stream,handle,_music:music,track})}
  pub fn set_track(&self,n:usize){self.track.store(n%3,Ordering::Relaxed);}
  pub fn sfx(&self,kind:u8){let Ok(sink)=Sink::try_new(&self.handle) else{return};let count=RATE as usize/7;let frequency=match kind{0=>660.0,1=>190.0,2=>440.0,_=>330.0};let mut samples=Vec::with_capacity(count);for i in 0..count{let t=i as f32/RATE as f32;let env=(1.0-i as f32/count as f32).powi(2);let sweep=frequency*(1.0+0.25*(1.0-i as f32/count as f32));samples.push((std::f32::consts::TAU*sweep*t).sin()*env*0.24);}sink.append(SamplesBuffer::new(1,RATE,samples));sink.detach();}
 }
}
#[cfg(feature="audio")] pub use live::Audio;
#[cfg(not(feature="audio"))] pub struct Audio;
#[cfg(not(feature="audio"))] impl Audio {pub fn new()->Option<Self>{None}pub fn set_track(&self,_:usize){}pub fn sfx(&self,_:u8){}}

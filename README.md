# Last Lantern

A complete, compact 2D pixel-art action adventure for Linux x86_64, written in Rust. You play the keeper of a fading flame, fighting through three haunted regions to defeat the Lantern Warden.

## Build and run

Rust stable (edition 2021) and a C toolchain are required. On Debian/Ubuntu, install the window/audio development libraries first:

```sh
sudo apt install build-essential pkg-config libx11-dev libxi-dev libxrandr-dev libxcursor-dev libxinerama-dev libasound2-dev
cargo build --release
cargo run --release
```

The window is rendered by a CPU framebuffer at **320×180** and presented at 1280×720 with integer nearest-neighbor scaling. OpenGL/Vulkan are not used. `--no-audio` runs without an audio device; `cargo build --release --no-default-features` also excludes Rodio if audio development headers are unavailable.

### Controls

- **Keyboard:** arrows or WASD to move, Space or Z to strike, P/Escape to pause/resume, M to save and return to title, Enter to select.
- **Linux gamepad:** the first legacy Linux joystick interface (`/dev/input/js0`) is polled non-blockingly. Stick axes move; button 0 attacks, button 1 confirms, button 3 pauses. The game remains fully playable without a controller.
- Settings adjust difficulty with left/right. Difficulty changes enemy health. Save data is written as `last-lantern.sav` in the current working directory.

### Headless renders and inspection

```sh
cargo run --release -- --headless --frames 12 --dump out/
cargo run --release -- --art-report out/
cargo run --release -- --benchmark
```

Headless mode writes `frame-0000.png` onward plus `contact-sheet.png` (four 320×180 views at 2× nearest-neighbor) and `art-report.txt`. The text report contains sprite dimensions, an authored-pixel palette histogram, and 80×45 ASCII palette-index down-samples of title, areas, and boss views. It opens no window and needs no audio device. `--benchmark` measures 1,000 CPU-rendered frames and prints the p99 time against the 16 ms target.

### Tests

```sh
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

The suite includes a palette/grid/flat-region/outline/renderer-primitive art linter, a deterministic 1,000-seed A* encounter simulation (prints win rate and mean encounter steps), LOS checks, and a 120-frame p99 game-update-plus-software-render timing test. The runtime `--benchmark` provides a longer 1,000-frame timing check.

## Game content

- **Areas:** The Mossen Mire, the Sunken Belfry, and the Warden's Hollow. Each has its own hue-shifted palette treatment, parallax silhouettes, and tile variants.
- **Enemies:** Wisps, Mosslings, Stalkers, Brutes, and the Lantern Warden boss. Wisp/Mossling/Stalker/Bat/Brute/Warden art uses authored animation frames where motion calls for them; sprites are palette-indexed pixel grids.
- **Progression:** defeated enemies drop ember shards; three shards grant a maximum-heart upgrade and heal one heart. The boss is the final encounter. Progress autosaves on transitions, pause, and exit.
- **Game feel:** short hit-stop, restrained screen shake, sprite-pixel impact particles, invulnerability flicker, and responsive synthesized hit/damage cues.

## Art and renderer

All visible game art is authored as explicit character grids in `src/assets.rs` and mapped to a fixed 16-color hue-shifted palette. The bitmap typeface is explicitly authored in the same source file. Runtime drawing is indexed-pixel sprite blitting with optional flips and palette swaps; screen clearing and framebuffer presentation are the only non-sprite operations. There is no vector art, anti-aliasing, shape drawing, filtering, 3D geometry, or procedural visible texture generation. Tile placement, parallax offsets, animation frame selection, and particles choose/blit authored pixels.

## AI

Navigation uses the public `pathfinding` crate's A* implementation. The game-specific policy adds:

1. Grid LOS checks to decide whether a target is currently visible.
2. Hearing-based investigation of nearby movement/combat, with a separate shorter memory window.
3. Last-seen target memory and investigation after sight is lost.
4. Role tactics: flankers approach an offset, hunters focus the shared player target, and guards hold position until the player is seen.
5. Low-health retreat behavior.
6. Difficulty-scaled encounter HP and type-specific, tuned reaction delays.

The seeded verification simulation exercises the same A* helper and asserts every encounter reaches its goal without a stuck agent, path failure, or crash; A* state is integer-grid-only, so NaN coordinates are unrepresentable.

## Audio

Music is **original and code-composed** in Rust: three tracker-style modal note sequences (title, exploration, boss) are synthesized at runtime with a small oscillator and envelope, and switched by game state. Short attack and damage effects are also synthesized in code. No recorded samples or copied melodies are included. Rodio is optional at compile time through Cargo features and output-device failure is non-fatal.

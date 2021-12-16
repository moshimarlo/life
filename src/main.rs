mod state;
use state::State;

use ggez::{ContextBuilder, conf, event};

pub const GRID_SIZE: (usize, usize) = (100, 100);
pub const GRID_CELL_SIZE: (u32, u32) = (12, 12);
pub const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);
pub const DESIRED_FPS: u32 = 30;

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("life", "Mikko")
        .window_setup(conf::WindowSetup::default().title("Conway's Game of Life"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .unwrap();
    let mut ctx = ctx;
    let state = State::new(&mut ctx);
    event::run(ctx, event_loop, state);
}
use crate::*;

use ggez::{
    Context,
    event::{EventHandler, KeyCode, KeyMods, MouseButton},
    error::{GameError, GameResult},
    timer,
    graphics::{self, Color, Mesh, DrawMode, Rect},
    mint,
};

use rand::Rng;
pub struct State {
    board: Vec<u8>,
    board_next: Vec<u8>,
    meshes_alive: Vec<Mesh>,
    meshes_dead: Vec<Mesh>,
    rects: Vec<Rect>,
    paused: bool,
}

impl State {
    pub fn new(ctx: &mut Context) -> Self {
        let mut s = Self {
            board: vec![],
            board_next: vec![],
            meshes_alive: vec![],
            meshes_dead: vec![],
            rects: vec![],
            paused: true,
        };
        let mut rng = rand::thread_rng();
        let w = GRID_SIZE.0 as i32;
        for i in 0..GRID_SIZE.0 as usize {
            for j in 0..GRID_SIZE.1 as usize {
                s.rects.push(Rect::new(
                    i as f32 * GRID_CELL_SIZE.0 as f32,
                    j as f32 * GRID_CELL_SIZE.1 as f32,
                    GRID_CELL_SIZE.0 as f32,
                    GRID_CELL_SIZE.1 as f32
                ));
            }
        }
        for i in 0..GRID_SIZE.0 as usize {
            for j in 0..GRID_SIZE.1 as usize {
                let coords = (j as i32 * w + i as i32) as usize;
                let one_or_zero = rng.gen::<bool>() as u8;
                s.board.push(one_or_zero);
                s.board_next.push(one_or_zero);
                let mesh = match Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    s.rects[coords],
                    Color::BLACK
                ) {
                    Ok(mesh) => mesh,
                    Err(_) => panic!("Error on creating mesh"),
                };
                s.meshes_alive.push(mesh);

                let mesh = match Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    s.rects[coords],
                    Color::WHITE
                ) {
                    Ok(mesh) => mesh,
                    Err(_) => panic!("Error on creating mesh"),
                };
                s.meshes_dead.push(mesh);
            }
        }
        s
    }

    fn toggle_cell(&mut self, x: usize, y: usize) {
        self.board[y * GRID_SIZE.0 + x] ^= 0b0000_0001;
    } 

    fn check_neighbours(&mut self) {
        let w = GRID_SIZE.0 as i32;
        let h = GRID_SIZE.1 as i32;
        for i in 0..GRID_SIZE.0 {
            for j in 0..GRID_SIZE.1 {
                let coords = (j as i32 * w + i as i32) as usize;
                let mut count = 0;
                for x in i as i32 - 1..i as i32 + 2 {
                    for y in j as i32 - 1..j as i32 + 2 {
                        if self.board[((((y + h) % h)) * w + ((x + w) % w)) as usize] == 1 {
                            count += 1;
                        }
                    }
                }
                if self.board[coords] == 1 {
                    count -= 1;
                }
                match self.board[coords] {
                    1 => {
                        self.board_next[coords] = 1;
                        if count < 2 || count > 3 {
                            self.board_next[coords] = 0;
                        }
                    },
                    0 => {
                        self.board_next[coords] = 0;
                        if count == 3 {
                            self.board_next[coords] = 1;
                        }
                    },
                    _ => { panic!(); },
                }
            }
        }
    }

    fn update_state(&mut self) {
        let w = GRID_SIZE.0 as i32;
        for i in 0..GRID_SIZE.0 {
            for j in 0..GRID_SIZE.1 {
                let coords = (j as i32 * w + i as i32) as usize;
                self.board[coords] = self.board_next[coords];
            }
        }
    }

    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let w = GRID_SIZE.0 as i32;
        for i in 0..GRID_SIZE.0 as i32 {
            for j in 0..GRID_SIZE.1 as i32 {
                let alive = rng.gen::<bool>() as u8;
                self.board[(j * w + i) as usize] = alive;
                self.board_next[(j * w + i) as usize] = alive;
            }
        }
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            if !self.paused {
                self.check_neighbours();
                self.update_state();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());
        let w = GRID_SIZE.0 as i32;
        for i in 0..GRID_SIZE.0 {
            for j in 0..GRID_SIZE.1 {
                let coords = (j as i32 * w + i as i32) as usize;
                match self.board[coords] {
                    1 => {
                        graphics::draw(
                            ctx, 
                            &self.meshes_alive[coords],
                            (mint::Point2 {x: i as f32, y: j as f32},
                        ))?;
                    }
                    0 => {
                        graphics::draw(ctx,
                            &self.meshes_dead[coords],
                            (mint::Point2 {x: i as f32, y: j as f32},
                        ))?;
                    }
                    _ => { panic!(); }
                }
            }
        }
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Space => { self.paused = !self.paused; },
            KeyCode::Escape => { ggez::event::quit(_ctx); },
            KeyCode::Key5 => { self.randomize(); },
            _ => {},
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32
    ) {
        let x_f = _x / GRID_CELL_SIZE.0 as f32;
        let y_f = _y / GRID_CELL_SIZE.1 as f32;
        self.toggle_cell(x_f as usize, y_f as usize);
    }
}
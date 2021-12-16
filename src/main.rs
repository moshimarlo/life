use ggez::{
    Context,
    ContextBuilder,
    event::{self, EventHandler, KeyCode, KeyMods, MouseButton},
    error::{GameError, GameResult},
    conf,
    timer,
    graphics::{self, Color, Mesh, DrawMode, Rect},
    mint,
};
use rand::Rng;

const GRID_SIZE: (usize, usize) = (100, 100);
const GRID_CELL_SIZE: (u32, u32) = (12, 12);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const DESIRED_FPS: u32 = 60;

#[derive(Copy, Clone)]
struct Cell {
    x: u32,
    y: u32,
    alive: bool,
    alive_next: bool,
    rect: Rect,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            alive: false,
            alive_next: false,
            rect: Rect::new(
                0. as f32 * GRID_CELL_SIZE.0 as f32,
                0. as f32 * GRID_CELL_SIZE.1 as f32,
                GRID_CELL_SIZE.0 as f32,
                GRID_CELL_SIZE.1 as f32
            ),
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let color: Color = match self.alive {
            true => ggez::graphics::Color::BLACK,
            false => ggez::graphics::Color::WHITE,
        };
        let tile = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            self.rect,
            color
        )?;
        graphics::draw(ctx, &tile, (mint::Point2 {x: self.x as f32, y: self.y as f32},))
    }
}

struct State {
    board: [[Cell; GRID_SIZE.0]; GRID_SIZE.1],
    paused: bool,
}

impl State {
    pub fn new() -> Self {
        let mut s = Self {
            board: [[Cell::new(); GRID_SIZE.0]; GRID_SIZE.1],
            paused: true,
        };
        let mut rng = rand::thread_rng();

        for i in 0..GRID_SIZE.0 as usize {
            for j in 0..GRID_SIZE.1 as usize {
                s.board[i][j].x = i as u32;
                s.board[i][j].y = j as u32;
                let alive = rng.gen::<bool>();
                s.board[i][j].alive = alive;
                s.board[i][j].alive_next = alive;
                s.board[i][j].rect = Rect::new(
                s.board[i][j].x as f32 * GRID_CELL_SIZE.0 as f32,
                s.board[i][j].y as f32 * GRID_CELL_SIZE.1 as f32,
                GRID_CELL_SIZE.0 as f32,
                GRID_CELL_SIZE.1 as f32
                );
            }
        }
        s
    }

    fn toggle_cell(&mut self, x: usize, y: usize) {
        self.board[x][y].alive = !self.board[x][y].alive;
    } 

    fn check_neighbours(&mut self) {
        let w = GRID_SIZE.0 as i32;
        let h = GRID_SIZE.1 as i32;
        for i in 0..GRID_SIZE.0 {
            for j in 0..GRID_SIZE.1 {
                let mut count = 0;
                for x in i as i32 - 1..i as i32 + 2 {
                    for y in j as i32 - 1..j as i32 + 2 {
                        if self.board[((x + w) % w) as usize][((y + h) % h) as usize].alive {
                            count += 1;
                        }
                    }
                }
                if self.board[i][j].alive {
                    count -= 1;
                }
                match self.board[i][j].alive {
                    true => {
                        self.board[i][j].alive_next = true;
                        if count < 2 || count > 3 {
                            self.board[i][j].alive_next = false;
                        }
                    }
                    false => {
                        self.board[i][j].alive_next = false;
                        if count == 3 {
                            self.board[i][j].alive_next = true;
                        }
                    }
                }
            }
        }
    }

    fn update_state(&mut self) {
        for i in 0..GRID_SIZE.0 {
            for j in 0..GRID_SIZE.1 {
                self.board[i][j].alive = self.board[i][j].alive_next;
            }
        }
    }

    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..GRID_SIZE.0 as usize {
            for j in 0..GRID_SIZE.1 as usize {
                let alive = rng.gen::<bool>();
                self.board[i][j].alive = alive;
                self.board[i][j].alive_next = alive;
            }
        }
    }
}

pub fn mod_floor(a: i32, max: usize) -> usize {
    let n = max as i32;
    (((a % n) + n) % n) as usize
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
        for i in 0..GRID_SIZE.0 {
            for j in 0..GRID_SIZE.1 {
                self.board[i][j].draw(ctx)?;
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

fn main() {
    let state = State::new();
    let (ctx, event_loop) = ContextBuilder::new("life", "Mikko")
        .window_setup(conf::WindowSetup::default().title("Conway's Game of Life"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .unwrap();
    event::run(ctx, event_loop, state);
}
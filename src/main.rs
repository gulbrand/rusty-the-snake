use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::{SystemTime, Duration};
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Game {
    snake_position: Vec<Position>,
    snake_direction: Direction,
    board_width: u32,
    board_height: u32
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let x = width as i32 / 2;
        let y = height as i32 / 2;
        let mut snake_positions: Vec<Position> = Vec::new();
        snake_positions.push(Position { x, y });
        snake_positions.push(Position { x: x + 1, y: y } );
        snake_positions.push(Position { x: x + 2, y: y } );
        Game {
            snake_position: snake_positions,
            snake_direction: Direction::RIGHT,
            board_width: width,
            board_height: height,
        }
    }

    pub fn turn(self: &mut Self, direction: Direction) {
        self.snake_direction = direction;
    }

    pub fn snake_head(self: &Self) -> Option<Position> {
        if self.snake_position.len() < 1 {
            return None;
        }
        let last = self.snake_position.last().unwrap();
        Some(Position {
            x: last.x,
            y: last.y
        })
    }

    fn _move_direction(self: &mut Self, dx: i32, dy: i32) {
        let head = self.snake_head().unwrap();
        let new_head_position = Position {
            x: head.x + dx,
            y: head.y + dy,
        };
        self.snake_position.remove(0);
        self.snake_position.push(new_head_position);
    }

    pub fn tick(self: &mut Self) {
        if self.snake_position.len() < 1 {
            return;
        }
        let direction = &self.snake_direction;
        match direction {
            Direction::RIGHT => {
                self._move_direction(1, 0);
            },
            Direction::LEFT => {
                self._move_direction(-1, 0);
            },
            Direction::UP => {
                self._move_direction(0, -1);
            },
            Direction::DOWN => {
                self._move_direction(0, 1);
            },
            _ => {},
        };
    }

    pub fn draw_game(self: &Self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for pos in self.snake_position.iter() {
            canvas.fill_rect(Rect::new(pos.x*16, pos.y*16, 16, 16));
        }
    }

}

#[cfg(test)]
pub mod tests {
    use crate::*;
    #[test]
    pub fn simple_test() {
        let width: u32 = 32;
        let height: u32 = 32;
        let x = width as i32 / 2;
        let y = height as i32 / 2;
        let initial_position = Position { x: x + 2, y };
        let mut game = Game::new(width, height);
        game.turn(Direction::UP);
        assert_eq!(game.snake_position.len(), 3);
        assert_eq!(game.snake_direction, Direction::UP);
        assert_eq!(game.snake_head(), Some(initial_position));
        game.turn(Direction::RIGHT);
        game.tick();
        let updated_position = Position {
            x: initial_position.x + 1,
            y: initial_position.y,
        };
        assert_eq!(game.snake_head(), Some(updated_position))
    }
}


fn main() {
    let SQUARE_SIZE = 16;
    let PLAY_AREA_WIDTH = 32;
    let PLAY_AREA_HEIGHT = 32;

    let mut game = Game::new(PLAY_AREA_WIDTH, PLAY_AREA_HEIGHT);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rusty-the-snake",
                SQUARE_SIZE * PLAY_AREA_WIDTH,
                      SQUARE_SIZE * PLAY_AREA_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string()).unwrap();

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string()).unwrap();


    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    game.draw_game(&mut canvas);
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_tick = SystemTime::now();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. }
                        => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    game.turn(Direction::DOWN);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    game.turn(Direction::RIGHT);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    game.turn(Direction::LEFT);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    game.turn(Direction::UP);
                },
                _ => ()
            }
        }
        if last_tick.elapsed().unwrap() > Duration::from_millis(600) {
            last_tick = SystemTime::now();
            game.tick();
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            game.draw_game(&mut canvas);
            canvas.present();
        }
    }
}

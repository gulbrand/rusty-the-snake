use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::{SystemTime, Duration};
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use rand::Rng;


#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
    board_height: u32,
    turn_command: Option<Direction>,
    fruit_position: Vec<Position>,
    fruit_spawn_probability: i32,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let x = width as i32 / 2;
        let y = height as i32 / 2;
        let mut snake_positions: Vec<Position> = Vec::new();
        snake_positions.push(Position { x, y });
        snake_positions.push(Position { x: x + 1, y: y } );
        snake_positions.push(Position { x: x + 2, y: y } );
        snake_positions.push(Position { x: x + 3, y: y } );
        snake_positions.push(Position { x: x + 4, y: y } );
        Game {
            snake_position: snake_positions,
            snake_direction: Direction::RIGHT,
            board_width: width,
            board_height: height,
            turn_command: None,
            fruit_position: Vec::new(),
            fruit_spawn_probability: 15,
        }
    }

    fn _get_allowable_turn_vec(self: &Self) -> Vec<Direction> {
        match self.snake_direction {
            Direction::RIGHT => vec![Direction::UP, Direction::DOWN],
            Direction::LEFT => vec![Direction::UP, Direction::DOWN],
            Direction::UP => vec![Direction::RIGHT, Direction::LEFT],
            Direction::DOWN => vec![Direction::RIGHT, Direction::LEFT]
        }
    }

    fn _apply_turn_command(self: &mut Self) {
        if self.turn_command.is_some() {
            println!("applying the turn command");
            self.snake_direction = self.turn_command.unwrap();
        }
    }

    pub fn turn(self: &mut Self, direction: Direction) {
        let allowable_turns = self._get_allowable_turn_vec();
        if allowable_turns.contains(&direction) {
            self.turn_command = Some(direction);
        }
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

    fn _move_snake(self: &mut Self) {
        if self.snake_position.len() < 1 {
            return;
        }
        self._apply_turn_command();
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
        };
    }

    pub fn _spawn_fruit_definitely(self: &mut Self) {
        let mut rng = rand::thread_rng();
        let rng_x = rng.gen_range(0, self.board_width);
        let rng_y = rng.gen_range(0, self.board_height);
        self.fruit_position.push(Position { x: rng_x as i32, y: rng_y as i32 });
    }

    pub fn _spawn_fruit_maybe(self: &mut Self) {
        let mut rng = rand::thread_rng();
        let rng_num: i32 = rng.gen_range(0, 100);
        if rng_num <= self.fruit_spawn_probability {
            self._spawn_fruit_definitely();
        }
    }

    pub fn tick(self: &mut Self) {
        self._move_snake();
        self._spawn_fruit_maybe();
    }

    pub fn draw_game(self: &Self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for pos in self.snake_position.iter() {
            if let Err(result) = canvas.fill_rect(Rect::new(pos.x*16, pos.y*16, 16, 16)) {
                println!("error - {}", result);
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        for pos in self.fruit_position.iter() {
            if let Err(result) = canvas.fill_rect(Rect::new(pos.x*16, pos.y*16, 16, 16)) {
                println!("error - {}", result);
            }
        }
    }

    pub fn is_game_over(self: &Self) -> bool {
        let snake_head_pos = self.snake_head().unwrap();
        let snake_x = snake_head_pos.x;
        let snake_y = snake_head_pos.y;
        if snake_y < 0 || snake_y >= self.board_height as i32 {
            return true;
        }
        if snake_x < 0 || snake_x >= self.board_width as i32 {
            return true;
        }
        for (i, pos) in self.snake_position.iter().enumerate() {
            if i != self.snake_position.len()-1 && pos.x == snake_x && pos.y == snake_y {
                return true;
            }
        }
        false
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
        let initial_position = Position { x: x + 4, y };
        let mut game = Game::new(width, height);
        assert_eq!(game.snake_position.len(), 5);
        assert_eq!(game.snake_head(), Some(initial_position));
        game.turn(Direction::RIGHT);
        game.tick();
        let updated_position = Position {
            x: initial_position.x + 1,
            y: initial_position.y,
        };
        assert_eq!(game.snake_head(), Some(updated_position));

        // turn mechanics
        assert_eq!(game.snake_direction, Direction::RIGHT);
        game.turn(Direction::LEFT);
        assert_eq!(game.snake_direction, Direction::RIGHT);
        game.tick();
        assert_eq!(game.snake_direction, Direction::RIGHT);

        game.turn(Direction::UP);
        assert_eq!(game.snake_direction, Direction::RIGHT);
        game.tick();
        assert_eq!(game.snake_direction, Direction::UP);

    }

    #[test]
    pub fn border_test() {
        let width = 7;
        let height = 7;
        let mut game = Game::new(width, height);
        println!("{:?}", game);

        game.tick();
        game.tick();
        game.tick();
        game.tick();
        game.tick();
        println!("{:?}", game);
        assert_eq!(game.is_game_over(), true);
    }
}

fn main() {
    let square_size = 16;
    let play_area_width = 32;
    let play_area_height = 32;

    let mut game = Game::new(play_area_width, play_area_height);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rusty-the-snake",
                square_size * play_area_width,
                square_size * play_area_height)
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
        if game.is_game_over() {
            break 'running;
        }
    }
}

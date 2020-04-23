use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::{SystemTime, Duration};
use sdl2::render::{WindowCanvas, TextureQuery};
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use rand::Rng;
use std::collections::VecDeque;
use std::path::Path;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

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
    snake_position: VecDeque<Position>,
    snake_direction: Direction,
    snake_tail_last_pos: Position,
    board_width: u32,
    board_height: u32,
    turn_command: Option<Direction>,
    fruit_position: Vec<Position>,
    fruit_spawn_probability: i32,
    player_score: u32
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let x = width as i32 / 2;
        let y = height as i32 / 2;
        let mut snake_positions: VecDeque<Position> = VecDeque::new();
        snake_positions.push_back(Position { x, y });
        snake_positions.push_back(Position { x: x + 1, y } );
        snake_positions.push_back(Position { x: x + 2, y } );
        snake_positions.push_back(Position { x: x + 3, y } );
        snake_positions.push_back(Position { x: x + 4, y } );
        Game {
            snake_position: snake_positions,
            snake_direction: Direction::RIGHT,
            snake_tail_last_pos: Position { x, y },
            board_width: width,
            board_height: height,
            turn_command: None,
            fruit_position: Vec::new(),
            fruit_spawn_probability: 15,
            player_score: 0,
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
        let last = self.snake_position.back().unwrap();
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
        self.snake_tail_last_pos = self.snake_position.remove(0).unwrap();
        self.snake_position.push_back(new_head_position);
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

    fn _is_snake_on_position(self: &Self, pos: &Position) -> bool {
        for snake_pos in self.snake_position.iter() {
            if pos.x == snake_pos.x && pos.y == snake_pos.y {
                return true;
            }
        }
        return false;
    }

    pub fn _spawn_fruit_definitely(self: &mut Self) {
        loop {
            let mut rng = rand::thread_rng();
            let rng_x = rng.gen_range(0, self.board_width);
            let rng_y = rng.gen_range(0, self.board_height);
            let fruit_position_candidate = Position { x: rng_x as i32, y: rng_y as i32 };
            if !self._is_snake_on_position(&fruit_position_candidate) {
                self.fruit_position.push(Position { x: rng_x as i32, y: rng_y as i32 });
                break;
            }
        }
    }

    pub fn _spawn_fruit_maybe(self: &mut Self) {
        if self.fruit_position.len() < 10 {
            let mut rng = rand::thread_rng();
            let rng_num: i32 = rng.gen_range(0, 100);
            if rng_num <= self.fruit_spawn_probability {
                self._spawn_fruit_definitely();
            }
        }
    }

    pub fn _eat_fruit_maybe(self: &mut Self) {
        let snake_head = self.snake_head().unwrap();
        for (i, pos) in self.fruit_position.iter().enumerate() {
            if snake_head.x == pos.x && snake_head.y == pos.y {
                self.fruit_position.remove(i);
                self.snake_position.push_front(self.snake_tail_last_pos);
                self.player_score += 1;
                break;
            }
        }
    }

    pub fn tick(self: &mut Self) {
        self._move_snake();
        self._eat_fruit_maybe();
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

    pub fn get_snake_hz(self: &Self) -> Duration {
        let base_rate = 300;
        let length_modified_rate = base_rate - (self.snake_position.len() * 12);
        return Duration::from_millis(length_modified_rate as u64);
    }
    pub fn player_score(self: &Self) -> u32 {
        self.player_score
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
    // https://www.fontsquirrel.com/license/droid-sans-mono
    let font_path: &Path = Path::new("./DroidSansMono.ttf");
    println!("linked sdl2_ttf: {}", sdl2::ttf::get_linked_version());
    let square_size = 16;
    let play_area_width = 32;
    let play_area_height = 32;

    let mut game = Game::new(play_area_width, play_area_height);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let mut font = ttf_context.load_font(font_path, 15).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    let score_surface =
        font.render(format!("Score: {}", 0).as_str())
            .blended(Color::RGBA(255, 0, 0, 255)).map_err(|e| e.to_string()).unwrap();
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

    let texture_creator = canvas.texture_creator();
    let score_texture = texture_creator.create_texture_from_surface(&score_surface)
        .map_err(|e| e.to_string()).unwrap();
    let TextureQuery { width, height, .. } = score_texture.query();
    let padding = 64;
    let target = rect!(0, 0, width, height);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    game.draw_game(&mut canvas);
    canvas.copy(&score_texture, None, Some(target)).unwrap();
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
        if last_tick.elapsed().unwrap() > game.get_snake_hz() {
            last_tick = SystemTime::now();
            game.tick();
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            game.draw_game(&mut canvas);
            let score_surface =
                font.render(format!("Score: {}", game.player_score()).as_str())
                    .blended(Color::RGBA(255, 0, 0, 255)).map_err(|e| e.to_string()).unwrap();
            let texture_creator = canvas.texture_creator();
            let score_texture = texture_creator.create_texture_from_surface(&score_surface)
                .map_err(|e| e.to_string()).unwrap();
            let TextureQuery { width, height, .. } = score_texture.query();
            let padding = 64;
            let target = rect!(0, 0, width, height);
            canvas.copy(&score_texture, None, Some(target)).unwrap();
            canvas.present();
        }
        if game.is_game_over() {
            break 'running;
        }
    }
    println!("final score: {}", game.player_score());
}

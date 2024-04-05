#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use rand::{thread_rng, Rng};
use serde::Serialize;

const FIELD_WIDTH: usize = 25;
const FIELD_HEIGHT: usize = 25;

pub struct GameState(Mutex<Snake>);
pub struct Snake {
    field: Field,
    snake_body: SnakeBody,
    snake_head: Point,
    food_location: Point,
    direction: CurrentDirection,
    current_game: GameStatus,
    score: i32,
}
impl Snake {
    fn new() -> Self {
        Self {
            field: Field::new(),
            snake_body: SnakeBody::new(vec![Point(3, 3)]),
            snake_head: Point(3, 3),
            food_location: Point(4, 3),
            direction: CurrentDirection::new('d'),
            current_game: GameStatus::Starting,
            score: 0,
        }
    }
    fn update_snake(&mut self, direction: CurrentDirection) {
        if direction.current != self.direction.opposite() || self.snake_body.value.len() == 1 {
            self.direction.current = direction.current;
        }

        let new_part = match self.direction.current {
            Direction::Up => Point(self.snake_head.0, self.snake_head.1 - 1),
            Direction::Right => Point(self.snake_head.0 + 1, self.snake_head.1),
            Direction::Down => Point(self.snake_head.0, self.snake_head.1 + 1),
            Direction::Left => Point(self.snake_head.0 - 1, self.snake_head.1),
        };
        if new_part.1 == FIELD_HEIGHT as i32
            || new_part.1 < 0
            || new_part.0 == FIELD_WIDTH as i32
            || new_part.0 < 0
        {
            self.current_game = GameStatus::Ended(GameEnd::Lose);
            return;
        }
        for (index, point) in self.snake_body.value.iter().enumerate() {
            if *point == self.snake_head && index != 0 {
                self.current_game = GameStatus::Ended(GameEnd::Lose);
                return;
            }
        }

        self.snake_body.append_to_body(new_part);

        if !(new_part == self.food_location) {
            self.snake_body.value.pop();
        } else {
            if !(self.snake_body.value.len() == FIELD_HEIGHT * FIELD_WIDTH) {
                self.food_location = {
                    let mut new_point: Point = Point(0, 0);
                    let mut my_rng = thread_rng();
                    let mut found = false;
                    while !found {
                        new_point = Point(
                            my_rng.gen_range(0..FIELD_WIDTH as i32),
                            my_rng.gen_range(0..FIELD_HEIGHT as i32),
                        );
                        if !self.snake_body.value.contains(&new_point) {
                            found = true;
                        }
                    }
                    new_point
                };
                self.score += 1;
            } else {
                self.current_game = GameStatus::Ended(GameEnd::Win)
            }
        }
        self.snake_head = *self.snake_body.value.first().unwrap();
    }
    fn change_direction(&mut self, direction: char) {
        self.direction.change_current_direction(direction);
    }
    fn start_game(&mut self) {
        self.current_game = GameStatus::Ongoing;
    }
    fn setup(&mut self) {
        self.field = Field::new();
        self.snake_body = SnakeBody::new(vec![Point(3, 3)]);
        self.snake_head = Point(3, 3);
        self.food_location = Point(4, 3);
        self.direction = CurrentDirection::new('d');
        self.current_game = GameStatus::Starting;
        self.score = 0
    }
    fn return_computed_field(&mut self) -> [[FieldTile; FIELD_WIDTH]; FIELD_HEIGHT] {
        let field = self
            .field
            .populate_field(&self.snake_body, &self.food_location);
        field
    }
}

#[derive(PartialEq, Eq)]
enum GameStatus {
    Starting,
    Ongoing,
    Ended(GameEnd),
}
#[derive(PartialEq, Eq)]
enum GameEnd {
    Win,
    Lose,
}
struct SnakeBody {
    value: Vec<Point>,
}
impl SnakeBody {
    fn new(body: Vec<Point>) -> Self {
        Self { value: body }
    }
    fn append_to_body(&mut self, part: Point) {
        self.value.insert(0, part);
    }
}

#[derive(Clone, Copy, Serialize)]
struct Field {
    value: [[FieldTile; FIELD_WIDTH]; FIELD_HEIGHT],
}
impl Field {
    fn new() -> Self {
        Self {
            value: [[FieldTile::EMPTY; FIELD_WIDTH]; FIELD_HEIGHT],
        }
    }
    fn populate_field(
        &mut self,
        snake_body: &SnakeBody,
        food_location: &Point,
    ) -> [[FieldTile; 25]; 25] {
        let snake = &snake_body.value;
        let mut field = self.value;
        for part in snake.into_iter() {
            field[part.1 as usize][part.0 as usize] = FieldTile::SNAKE;
        }
        field[food_location.1 as usize][food_location.0 as usize] = FieldTile::FOOD;
        field
    }
}
#[derive(Serialize, Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'w' => Self::Up,
            'd' => Self::Right,
            's' => Self::Down,
            'a' => Self::Left,
            _ => Self::Right,
        }
    }
}
#[derive(Serialize, Clone, Copy, Debug, PartialEq)]
struct CurrentDirection {
    current: Direction,
}

impl CurrentDirection {
    fn new(character: char) -> Self {
        Self {
            current: Direction::from(character),
        }
    }
    fn change_current_direction(&mut self, direction: char) {
        self.current = Direction::from(direction)
    }
    fn opposite(&self) -> Direction {
        return match self.current {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        };
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point(i32, i32);

#[derive(Serialize, Clone, Copy)]
enum FieldTile {
    EMPTY,
    FOOD,
    SNAKE,
}

#[tauri::command]
fn initialize_field(
    state: tauri::State<'_, GameState>,
) -> Result<[[FieldTile; FIELD_WIDTH]; FIELD_HEIGHT], String> {
    Ok(state.0.lock().unwrap().return_computed_field())
}
#[tauri::command]
fn update_snake(
    state: tauri::State<'_, GameState>,
    direction: char,
) -> Result<[[FieldTile; 25]; 25], String> {
    state
        .0
        .lock()
        .unwrap()
        .update_snake(CurrentDirection::new(direction));
    Ok(state.0.lock().unwrap().return_computed_field())
}
#[tauri::command]
fn change_direction(state: tauri::State<'_, GameState>, direction: char) -> Result<(), String> {
    state.0.lock().unwrap().change_direction(direction);
    Ok(())
}

#[tauri::command]
fn return_game_state(state: tauri::State<'_, GameState>) -> Result<String, String> {
    let game_status = match &state.0.lock().unwrap().current_game {
        GameStatus::Starting => "Starting",
        GameStatus::Ongoing => "Ongoing",
        GameStatus::Ended(GameEnd::Win) => "Win",
        GameStatus::Ended(GameEnd::Lose) => "Lose",
    };
    Ok(game_status.to_string())
}
#[tauri::command]
fn setup_game(state: tauri::State<'_, GameState>) -> Result<(), ()> {
    state.0.lock().unwrap().setup();
    Ok(())
}
#[tauri::command]
fn start_game(state: tauri::State<'_, GameState>) -> Result<(), ()> {
    state.0.lock().unwrap().start_game();
    Ok(())
}
#[tauri::command]
fn return_score(state: tauri::State<'_, GameState>) -> Result<i32, ()> {
    Ok(state.0.lock().unwrap().score)
}
fn main() {
    let new_snake = Snake::new();

    tauri::Builder::default()
        .manage(GameState(Mutex::new(new_snake)))
        .invoke_handler(tauri::generate_handler![
            initialize_field,
            change_direction,
            update_snake,
            return_game_state,
            setup_game,
            start_game,
            return_score
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

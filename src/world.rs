use crate::snake::{self, Snake};
use js_sys::Array;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Clone, PartialEq, Debug)]
#[wasm_bindgen]
pub struct World {
    world_size: i32,
    snake: Snake,
    price_cell: i32,
    score: i32,
}

#[wasm_bindgen]
impl World {
    pub fn new(size: i32, snake_length: i32) -> Self {
        init(size, snake_length)
    }
    pub fn can_play(&self)->bool{
        let head = self.snake.get_snake_head().unwrap();
         self.snake.contains_in_body(head)==false
    }
    pub fn reset_world(&mut self, size: i32, snake_length: i32) {
        let mut w = init(size, snake_length);
        self.price_cell = w.price_cell;
        self.score = 0;
        self.snake.set_direction(w.snake.get_direction());
        self.snake.clear_body();
        self.snake.append_body_cells(&mut w.snake.get_body());
        self.world_size = w.world_size;
    }
    pub fn set_snake_position(&mut self, pos: i32) {
        self.snake.set_position(pos)
    }
    pub fn append_snake_cell(&mut self) {
        self.snake.append_cell();
    }
    pub fn reset_price(&mut self) {
        let new_price = get_price(&self.snake, self.world_size);
        self.price_cell = new_price;
    }
    pub fn get_body(&self) -> Array {
        self.snake
            .get_body()
            .into_iter()
            .map(|item| JsValue::from(item.0))
            .collect()
    }

    pub fn get_snake_length(&self)->i32{
        self.snake.get_snake_length()
    }
    
    pub fn get_price_cell(&self) -> i32 {
        self.price_cell
    }
    pub fn set_snake_direction(&mut self, dir: snake::Direction) {
        let old_dir = self.snake.get_direction();
        if dir != snake::Direction::None {
            self.snake.set_direction(dir);
            let pos = self.calculate_pos();
            if pos == self.snake.get_body()[1].0 {
                self.snake.set_direction(old_dir);
            }
        }
    }
    pub fn move_snake(&mut self) {
        let pos = self.calculate_pos();
        self.snake.set_position(pos);
        self.check_price();

    }
    pub fn get_score(&self)->i32{
        self.score
    }
    fn check_price(&mut self) {
     if self.snake.get_snake_head().unwrap() == self.price_cell {
        let snake_length = self.snake.get_snake_length();
        self.append_snake_cell();
        self.reset_price();
        self.score += snake_length;
    }
    }

    fn calculate_pos(&mut self) -> i32 {
        let size = self.world_size * self.world_size;
        let head = self.snake.get_snake_head().unwrap();

        let col = head % self.world_size;
        let row = head / self.world_size;

        let direction = self.snake.get_direction();
        match direction {
            snake::Direction::Right => ((row * self.world_size) + (col + 1)) % size,
            snake::Direction::Left => {
                if col == 0 {
                    if row == 0 {
                        self.world_size * self.world_size
                    } else {
                        (((row - 1) * self.world_size) + (self.world_size - 1)) % size
                    }
                } else {
                    ((row * self.world_size) + (col - 1)) % size
                }
            }
            snake::Direction::Up => {
                if row == 0 {
                    if col == self.world_size - 1 {
                        ((self.world_size - 1) * self.world_size) % size
                    } else {
                        (((self.world_size - 1) * self.world_size) + (col + 1)) % size
                    }
                } else {
                    (((row - 1) * self.world_size) + col) % size
                }
            }
            snake::Direction::Down => {
                if row == self.world_size - 1 {
                    if col == 0 {
                        ((self.world_size * self.world_size) + (self.world_size - 1)) % size
                    } else {
                        ((self.world_size * self.world_size) + (col - 1)) % size
                    }
                } else {
                    (((row + 1) * self.world_size) + col) % size
                }
            }
            snake::Direction::None => head,
        }
    }
}

fn init(size: i32, snake_length: i32) -> World {
    let mut random = rand::thread_rng();
    let pos: i32 = random.gen_range(0..(size * size) - snake_length);
    let snake = Snake::new(pos, snake_length);
    let price = get_price(&snake, size);
    World {
        world_size: size,
        snake,
        price_cell: price,
        score: 0,
    }
}

fn get_price(snake: &Snake, size: i32) -> i32 {
    let mut random = rand::thread_rng();

    let mut price = random.gen_range(0..(size * size));
    loop {
        if snake.contains(price) {
            price = random.gen_range(0..(size * size));
        } else {
            break;
        }
    }
    price
}

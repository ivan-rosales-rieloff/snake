use crate::snake::Snake;
use rand::prelude::*;
#[derive(Clone, PartialEq, Debug)]
pub struct World {
    pub world_size: i32,
    pub snake: Snake,
    pub price_cell: i32,
    pub score: i32,
}
impl World {
    pub fn new(size: i32, snake_length: i32) -> Self {
        init(size, snake_length)
    }
    pub fn reset_world(&mut self, size: i32, snake_length: i32) {
        let mut w = init(size, snake_length);
        self.price_cell = w.price_cell;
        self.score = 0;
        self.snake.direction = w.snake.direction;
        self.snake.body.clear();
        self.snake.body.append(&mut w.snake.body);
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

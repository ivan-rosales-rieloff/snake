use std::option::Option;

impl Clone for Snake {
    fn clone(&self) -> Self {
        Self {
            body: self.body.clone(),
            direction: self.direction.clone(),
        }
    }
}
// impl Copy for Snake {
//     fn copy(&self) -> Self {
//         Self {
//             body: self.body.clone(),
//         }
//     }
// }

#[derive(PartialEq, Debug)]
pub struct Snake {
    pub body: Vec<SnakeCell>,
    pub direction: Direction,
}
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
// impl Copy for Snake {
//     fn clone(&self) -> Self {
//         self.clone()
//     }
// }

#[derive(Clone, PartialEq, Debug)]
pub struct SnakeCell(pub i32);

impl Snake {
    pub fn new(pos: i32, length: i32) -> Self {
        let mut cells = vec![];
        for p in 0..length {
            cells.push(SnakeCell(pos + p));
        }

        Self {
            body: cells,
            direction: Direction::Left,
        }
    }
    pub fn get_snake_head(&self) -> Option<i32> {
        if self.body.is_empty() {
            return Option::None;
        };
        Some(self.body[0].0)
    }
    pub fn get_snake_length(&self) -> i32 {
        self.body.len() as i32
    }
    pub fn contains(&self, pos: i32) -> bool {
        self.body.contains(&SnakeCell(pos))
    }
    pub fn contains_in_body(&self, pos: i32) -> bool {
        self.body[1..].contains(&SnakeCell(pos))
    }
    pub fn set_position(&mut self, pos: i32) {
        let old_body = self.body.clone();
        self.body[0].0 = pos;

        for e in 1..self.get_snake_length() {
            self.body[e as usize].0 = old_body[(e - 1) as usize].0;
        }
    }
    pub fn append_cell(&mut self) {
        let cell = self.body.last().unwrap().clone();
        self.body.push(cell);
    }
}

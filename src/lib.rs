use std::arch::global_asm;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize) -> Self {
        Self {
            body: vec![SnakeCell(spawn_index)],
            direction: Direction::Down,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> Self {
        Self {
            width,
            size: width * width,
            snake: Snake::new(snake_index),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    pub fn update(&mut self) {
        let snake_head_index = self.snake_head_index();
        // self.snake.body[0].0 = (snake_head_index + 1) % self.size;
        let (row, col) = self.index_to_cell(snake_head_index);

        let (row, col) = match self.snake.direction {
            Direction::Left => (row, (col - 1 + self.width) % self.width),
            Direction::Right => (row, (col + 1) % self.width),
            Direction::Up => ((row - 1 + self.width) % self.width, col),
            Direction::Down => ((row + 1) % self.width, col),
        };
        log(format!("{} {}", row, col));
        self.snake.body[0].0 = self.cell_to_index((row, col));
    }

    pub fn set_snake_head(&mut self, index: usize) {
        self.snake.body[0].0 = index
    }

    fn index_to_cell(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    fn cell_to_index(&self, cell: (usize, usize)) -> usize {
        cell.0 * self.width + cell.1
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

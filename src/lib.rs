

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/random.js")]
extern "C" {
    fn random(max: usize) -> usize;
    fn log(str: &str);
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

// [0] [1] [2] => [0] 根据方向变， [1] = 上一次的[0]以此类推
impl Snake {
    fn new(spawn_index: usize, size: usize) -> Self {
        let mut body = Vec::new();
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i))
        }
        Self {
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
#[warn(dead_code)]
pub struct World {
    width: usize,
    size: usize,
    reward_cell: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, spawn_index: usize) -> Self {
        let max = width * width;
        let snake = Snake::new(spawn_index, 3);
        Self {
            width,
            size: width * width,
            reward_cell: World::gen_reward_cell(max, &snake.body),
            snake,
            next_cell: None,
        }
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;
        loop {
            reward_cell = random(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }
        reward_cell
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);
        if self.snake.body[1].0 == next_cell.0 {
            return;
        } else {
            self.next_cell = Option::Some(next_cell);
        }
        self.snake.direction = direction;
    }

    pub fn update(&mut self) {
        let temp = self.snake.body.clone();

        match self.next_cell {
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            }
            None => {
                self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
            }
        }
        //如果当前头等于 蛋位置， 将temp最后一个元素填充到新的位置，并生成新的蛋
        let len = self.snake.body.len();
        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i - 1].0)
        }

        if self.snake.body[0].0 == self.reward_cell() {
            self.snake.body.push(temp[len - 1]);
            self.reward_cell = World::gen_reward_cell(self.width * self.width, &self.snake.body);
        }
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_index = self.snake_head_index();
        let row = snake_index / self.width;
        return match direction {
            Direction::Up => {
                let border_hold = snake_index - (row * self.width);
                log(format!("bor {}", border_hold.to_string().as_str()).as_str());
                log(format!("index {}", snake_index.to_string().as_str()).as_str());
                if snake_index == border_hold {
                    SnakeCell((self.size - self.width) + border_hold)
                } else {
                    SnakeCell(snake_index - self.width)
                }
            }
            Direction::Down => {
                let border_hold = snake_index + ((self.width - row) * self.width);
                if snake_index + self.width == border_hold {
                    SnakeCell(border_hold - (row + 1) * self.width)
                } else {
                    SnakeCell(snake_index + self.width)
                }
            }
            Direction::Left => {
                let border_hold = row * self.width;
                if snake_index == border_hold {
                    SnakeCell(border_hold + self.width - 1)
                } else {
                    SnakeCell(snake_index - 1)
                }
            }
            Direction::Right => {
                let border_hold = (row + 1) * self.width;
                if snake_index + 1 == border_hold {
                    SnakeCell(border_hold - self.width)
                } else {
                    SnakeCell(snake_index + 1)
                }
            }
        };
    }
}

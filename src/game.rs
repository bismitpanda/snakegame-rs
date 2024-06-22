use raylib::prelude::*;

use crate::{
    food::Food,
    snake::{Direction, Snake},
    CELL_COUNT,
};

pub struct Game<'a> {
    snake: Snake,
    food: Food<'a>,
    running: bool,
    pub score: i32,
    last_update_time: f64,
    eat_sound: &'a Sound<'a>,
    wall_sound: &'a Sound<'a>,
    allow_move: bool,
}

impl<'a> Game<'a> {
    pub fn new(
        handle: &RaylibHandle,
        food_texture: &'a Texture2D,
        eat_sound: &'a Sound<'a>,
        wall_sound: &'a Sound<'a>,
    ) -> Self {
        let snake = Snake::new();
        let food = Food::new(handle, &snake.body, food_texture);

        Self {
            snake,
            food,
            running: true,
            score: 0,
            eat_sound,
            wall_sound,
            last_update_time: 0.0,
            allow_move: false,
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        self.food.draw(draw_handle);
        self.snake.draw(draw_handle);
    }

    pub fn update(&mut self, handle: &RaylibHandle) {
        if self.running {
            self.snake.update();
            self.check_collision_with_food(handle);
            self.check_collision_with_edges(handle);
            self.check_collision_with_tail(handle);
        }
    }

    pub fn handle_input(&mut self, handle: &RaylibHandle) {
        if self.running {
            let current_time = handle.get_time();

            if current_time - self.last_update_time >= 0.2 {
                self.last_update_time = current_time;
                self.update(handle);
                self.allow_move = true;
            }

            if handle.is_key_pressed(KeyboardKey::KEY_UP)
                && !self.snake.direction.is_vert()
                && self.allow_move
            {
                self.snake.direction = Direction::Up;
                self.allow_move = false
            } else if handle.is_key_pressed(KeyboardKey::KEY_DOWN)
                && !self.snake.direction.is_vert()
                && self.allow_move
            {
                self.snake.direction = Direction::Down;
                self.allow_move = false
            } else if handle.is_key_pressed(KeyboardKey::KEY_LEFT)
                && !self.snake.direction.is_horiz()
                && self.allow_move
            {
                self.snake.direction = Direction::Left;
                self.allow_move = false
            } else if handle.is_key_pressed(KeyboardKey::KEY_RIGHT)
                && !self.snake.direction.is_horiz()
                && self.allow_move
            {
                self.snake.direction = Direction::Right;
                self.allow_move = false
            }
        } else if handle.is_key_pressed(KeyboardKey::KEY_ENTER) {
            self.running = true;
        }
    }

    pub fn check_collision_with_food(&mut self, handle: &RaylibHandle) {
        if self.snake.body[0] == self.food.pos {
            self.food.pos = Food::generate_random_pos(handle, &self.snake.body);
            self.snake.add_segment = true;
            self.score += 1;
            self.eat_sound.play();
        }
    }

    pub fn check_collision_with_edges(&mut self, handle: &RaylibHandle) {
        if self.snake.body[0].x as i32 == CELL_COUNT || self.snake.body[0].x == -1.0 {
            self.game_over(handle);
        }
        if self.snake.body[0].y as i32 == CELL_COUNT || self.snake.body[0].y == -1.0 {
            self.game_over(handle);
        }
    }

    pub fn check_collision_with_tail(&mut self, handle: &RaylibHandle) {
        let mut headless_body = self.snake.body.clone();
        headless_body.pop_front();

        if headless_body.contains(&self.snake.body[0]) {
            self.game_over(handle);
        }
    }

    pub fn game_over(&mut self, handle: &RaylibHandle) {
        self.snake.reset();
        self.food.pos = Food::generate_random_pos(handle, &self.snake.body);
        self.running = false;
        self.score = 0;
        self.wall_sound.play();
    }
}

use std::collections::VecDeque;

use raylib::prelude::*;

use crate::{CELL_COUNT, CELL_SIZE, OFFSET};

pub struct Food<'a> {
    pub pos: Vector2,
    texture: &'a Texture2D,
}

impl<'a> Food<'a> {
    pub fn new(
        handle: &RaylibHandle,
        snake_body: &VecDeque<Vector2>,
        texture: &'a Texture2D,
    ) -> Self {
        Self {
            pos: Self::generate_random_pos(handle, snake_body),
            texture,
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_texture(
            self.texture,
            OFFSET + (self.pos.x as i32) * CELL_SIZE,
            OFFSET + (self.pos.y as i32) * CELL_SIZE,
            Color::WHITE,
        )
    }

    pub fn generate_random_cell(handle: &RaylibHandle) -> Vector2 {
        Vector2::new(
            handle.get_random_value::<i32>(0..CELL_COUNT - 1) as f32,
            handle.get_random_value::<i32>(0..CELL_COUNT - 1) as f32,
        )
    }

    pub fn generate_random_pos(handle: &RaylibHandle, snake_body: &VecDeque<Vector2>) -> Vector2 {
        let mut pos = Self::generate_random_cell(handle);

        while snake_body.contains(&pos) {
            pos = Self::generate_random_cell(handle)
        }

        pos
    }
}

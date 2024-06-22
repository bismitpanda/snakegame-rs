use std::collections::VecDeque;

use raylib::prelude::*;

use crate::{CELL_SIZE, DARK_GREEN, GREEN, OFFSET};

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vector2(self) -> Vector2 {
        match self {
            Self::Up => Vector2::new(0.0, -1.0),
            Self::Down => Vector2::new(0.0, 1.0),
            Self::Left => Vector2::new(-1.0, 0.0),
            Self::Right => Vector2::new(1.0, 0.0),
        }
    }

    pub fn is_vert(self) -> bool {
        matches!(self, Self::Down | Self::Up)
    }

    pub fn is_horiz(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    pub fn eye_offset(self) -> ((f32, f32), (f32, f32)) {
        match self {
            Direction::Left => ((5.0, 5.0), (5.0, 20.0)),
            Direction::Up => ((20.0, 5.0), (5.0, 5.0)),
            Direction::Right => ((20.0, 5.0), (20.0, 20.0)),
            Direction::Down => ((20.0, 20.0), (5.0, 20.0)),
        }
    }
}

pub struct Snake {
    pub body: VecDeque<Vector2>,
    pub direction: Direction,
    pub add_segment: bool,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: VecDeque::from([
                Vector2::new(6.0, 9.0),
                Vector2::new(5.0, 9.0),
                Vector2::new(4.0, 9.0),
            ]),
            direction: Direction::Right,
            add_segment: false,
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        for (i, segment) in self.body.iter().enumerate() {
            draw_handle.draw_rectangle_rounded(
                Rectangle::new(
                    OFFSET as f32 + segment.x * CELL_SIZE as f32,
                    OFFSET as f32 + segment.y * CELL_SIZE as f32,
                    CELL_SIZE as f32,
                    CELL_SIZE as f32,
                ),
                0.5,
                6,
                DARK_GREEN,
            );

            if i == 0 {
                let (eye1, eye2) = self.direction.eye_offset();

                draw_handle.draw_rectangle_rounded(
                    Rectangle::new(
                        OFFSET as f32 + segment.x * CELL_SIZE as f32 + eye1.0,
                        OFFSET as f32 + segment.y * CELL_SIZE as f32 + eye1.1,
                        5.0,
                        5.0,
                    ),
                    0.5,
                    6,
                    GREEN,
                );

                draw_handle.draw_rectangle_rounded(
                    Rectangle::new(
                        OFFSET as f32 + segment.x * CELL_SIZE as f32 + eye2.0,
                        OFFSET as f32 + segment.y * CELL_SIZE as f32 + eye2.1,
                        5.0,
                        5.0,
                    ),
                    0.5,
                    6,
                    GREEN,
                );
            }
        }
    }

    pub fn update(&mut self) {
        self.body
            .push_front(self.body[0] + self.direction.to_vector2());

        if self.add_segment {
            self.add_segment = false;
        } else {
            self.body.pop_back();
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

mod food;
mod game;
mod snake;

use game::Game;
use raylib::prelude::*;

pub const GREEN: Color = Color::new(173, 204, 96, 255);
pub const DARK_GREEN: Color = Color::new(43, 51, 24, 255);

pub const CELL_SIZE: i32 = 30;
pub const CELL_COUNT: i32 = 25;
pub const OFFSET: i32 = 75;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(
            2 * OFFSET + CELL_SIZE * CELL_COUNT,
            2 * OFFSET + CELL_SIZE * CELL_COUNT,
        )
        .title("Snake Game")
        .build();

    let ra = RaylibAudio::init_audio_device().unwrap();

    let food_texture = rl.load_texture(&thread, "assets/images/food.png").unwrap();
    let eat_sound = ra.new_sound("assets/sounds/eat.mp3").unwrap();
    let wall_sound = ra.new_sound("assets/sounds/wall.mp3").unwrap();

    rl.set_target_fps(60);

    let mut game = Game::new(&rl, &food_texture, &eat_sound, &wall_sound);

    while !rl.window_should_close() {
        game.handle_input(&rl);

        let mut draw_handle = rl.begin_drawing(&thread);

        draw_handle.clear_background(GREEN);
        draw_handle.draw_rectangle_lines_ex(
            Rectangle::new(
                OFFSET as f32 - 5.0,
                OFFSET as f32 - 5.0,
                CELL_SIZE as f32 * CELL_COUNT as f32 + 10.0,
                CELL_SIZE as f32 * CELL_COUNT as f32 + 10.0,
            ),
            5.0,
            DARK_GREEN,
        );
        draw_handle.draw_text("snake", OFFSET - 5, 20, 40, DARK_GREEN);
        draw_handle.draw_text(
            &format!("{}", game.score),
            OFFSET - 5,
            OFFSET + CELL_SIZE * CELL_COUNT + 10,
            40,
            DARK_GREEN,
        );

        game.draw(&mut draw_handle);
    }
}

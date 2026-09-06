use raylib::prelude::*;
use std::time::{Duration, Instant};

use crate::detect_music::is_audio_playing;

mod detect_music;

// fn rotate_teto(current_angle: f32, is_playing: bool, speed: f32) -> f32 {
//     // If music is NOT playing and we are already at 0, don't move
//     if !is_playing && current_angle == 0.0 {
//         return 0.0;
//     }

//     // Advance rotation
//     let mut new_angle = current_angle + speed;

//     // Wrap around 360 degrees
//     if new_angle >= 360.0 {
//         new_angle -= 360.0;
//     }

//     // If music stopped and advancing crossed or reached 0 deg, snap directly to 0
//     if !is_playing && new_angle < speed {
//         return 0.0;
//     }

//     new_angle
// }

fn rotate_teto(current_angle: f32, is_playing: bool, speed: f32) -> f32 {
    if !is_playing {
        return current_angle;
    }

    let mut new_angle = current_angle + speed;
    if new_angle >= 360.0 {
        new_angle -= 360.0;
    }

    new_angle
}

fn main() {
    const SCREEN_WIDTH: i32 = 240;
    const SCREEN_HEIGHT: i32 = 240;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("teeeeeeto")
        .transparent()
        .undecorated()
        .topmost()
        .build();

    if let Ok(icon) = raylib::texture::Image::load_image("./assets/teto1.png") {
        rl.set_window_icon(&icon);
    } else {
        eprintln!("Failed to load window icon image!");
    }

    let monitor: i32 = 0;
    let monitor_width: i32 = core::window::get_monitor_width(monitor);
    let monitor_height: i32 = core::window::get_monitor_height(monitor);

    let initial_pos_x: i32 = monitor_width - SCREEN_WIDTH;
    let initial_pos_y: i32 = monitor_height - SCREEN_HEIGHT;
    rl.set_window_position(initial_pos_x, initial_pos_y);
    rl.set_target_fps(20);

    let teto_img = rl.load_texture(&thread, "./assets/teto1.png").unwrap();

    let mut is_dragging: bool = false;
    let mut drag_offset: Vector2 = Vector2::zero();

    let mut rotation_angle: f32 = 0.0;
    let mut is_playing: bool = false;
    let mut last_audio_check = Instant::now();
    let check_interval = Duration::from_millis(200);

    while !rl.window_should_close() {
        let mouse_pos: Vector2 = rl.get_mouse_position();
        
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if mouse_pos.x >= 0.0 && mouse_pos.x <= SCREEN_WIDTH as f32 && mouse_pos.y >= 0.0 && mouse_pos.y <= SCREEN_HEIGHT as f32 {
                is_dragging = true;
                drag_offset = mouse_pos;
            }
        }

        if is_dragging {
            if rl.is_mouse_button_up(MouseButton::MOUSE_BUTTON_LEFT){
                is_dragging = false;
            }

            let win_pos: Vector2 = rl.get_window_position();

            let new_win_x: f32 = win_pos.x + mouse_pos.x - drag_offset.x;
            let new_win_y: f32 = win_pos.y + mouse_pos.y - drag_offset.y;

            println!("x:{} -> {}, y:{} -> {}, target: {} {}", win_pos.x, new_win_x, win_pos.y, new_win_y, new_win_x, new_win_y);
            rl.set_window_position(new_win_x as i32, new_win_y as i32);
        }

        if last_audio_check.elapsed() >= check_interval {
            is_playing = is_audio_playing().unwrap_or(false);
            last_audio_check = Instant::now();
        }

        rotation_angle = rotate_teto(rotation_angle, is_playing, 10.0);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLANK);
        // d.draw_texture(&teto_img, 10, 10, Color::WHITE);

        let source_rec = Rectangle::new(0.0, 0.0, teto_img.width as f32, teto_img.height as f32);
        let dest_rec = Rectangle::new(
            SCREEN_WIDTH as f32 / 2.0,
            SCREEN_HEIGHT as f32 / 2.0,
            teto_img.width as f32,
            teto_img.height as f32,
        );
        let origin = Vector2::new(teto_img.width as f32 / 2.0, teto_img.height as f32 / 2.0);

        d.draw_texture_pro(&teto_img, source_rec, dest_rec, origin, rotation_angle, Color::WHITE);
    }
}
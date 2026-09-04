use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Window Test Project")
        .resizable()
        .build();
    let mut window_height: i32;
    let mut window_width: i32;
    const FONT_SIZE: i32 = 20;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // Get window size
        window_height = d.get_screen_height();
        window_width = d.get_screen_width();

        // Get window center
        let window_center_x: i32 = window_width / 2;
        let window_center_y: i32 = window_height / 2;

        d.clear_background(Color::WHITE);

        // Draw perpendicular lines to indicate center
        d.draw_line(window_center_x, 0, window_center_x, window_height, Color::CYAN);
        d.draw_line(0, window_center_y, window_width, window_center_y, Color::CYAN);

        // Draw centered text
        let window_height_text: &String = &format!("Window Height: {}", window_height);
        let window_height_text_width: i32 = d.measure_text(window_height_text, FONT_SIZE);
        d.draw_text(
            window_height_text,
            (window_width - window_height_text_width) / 2,
            window_height / 2 - FONT_SIZE as i32,
            FONT_SIZE,
            Color::BLACK,
        );

        // Draw centered text
        let window_width_text: &String = &format!("Window Width: {}", window_width);
        let window_width_text_width: i32 = d.measure_text(window_width_text, FONT_SIZE);
        d.draw_text(
            window_width_text,
            (window_width - window_width_text_width) / 2,
            window_height / 2,
            FONT_SIZE,
            Color::BLACK,
        );
    }
}

// How to make a resizeable window

// How to make a fixed window (size, position)
// How to make window stay on top
// How to edit window styles (color, border, etc)
// How to make window transparent

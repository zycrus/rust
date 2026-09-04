use raylib::prelude::*;
use ksni::TrayMethods; // Import the trait to get .spawn()

struct TetoTray;

impl ksni::Tray for TetoTray {
    fn icon_name(&self) -> String {
        "user-available".into()
    }

    fn id(&self) -> String {
        "teeeeeeto-pet".into()
    }

    fn title(&self) -> String {
        "Teto Desktop Pet".into()
    }

    fn category(&self) -> ksni::Category {
        ksni::Category::ApplicationStatus
    }
}

fn main() {
    let _handle = TetoTray.spawn();

    const SCREEN_WIDTH: i32 = 240;
    const SCREEN_HEIGHT: i32 = 240;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Teeeeeeto")
        .transparent()
        .undecorated()
        .topmost()
        .build();

    let _ = std::process::Command::new("xdotool")
        .args([
            "search", "--onlyvisible", "--name", "Teeeeeeto",
            "windowstate", "--add", "SKIP_TASKBAR",
            "windowstate", "--add", "SKIP_PAGER"
        ])
        .status();

    let monitor: i32 = 0;
    let monitor_width: i32 = core::window::get_monitor_width(monitor);
    let monitor_height: i32 = core::window::get_monitor_height(monitor);

    let initial_pos_x: i32 = monitor_width - SCREEN_WIDTH;
    let initial_pos_y: i32 = monitor_height - SCREEN_HEIGHT;
    rl.set_window_position(initial_pos_x, initial_pos_y);
    rl.set_target_fps(30);

    let teto_img = rl.load_texture(&thread, "./assets/teto1.png").unwrap();

    let mut is_dragging: bool = false;
    let mut drag_offset: Vector2 = Vector2::zero();

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos: Vector2 = rl.get_mouse_position();

            if mouse_pos.x >= 0.0 && mouse_pos.x <= SCREEN_WIDTH as f32 && mouse_pos.y >= 0.0 && mouse_pos.y <= SCREEN_HEIGHT as f32 {
                is_dragging = true;
                drag_offset = mouse_pos;
            }
        }

        if is_dragging {
            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                let win_pos: Vector2 = rl.get_window_position();
                let mouse_pos: Vector2 = rl.get_mouse_position();

                let new_win_x: f32 = win_pos.x + mouse_pos.x - drag_offset.x;
                let new_win_y: f32 = win_pos.y + mouse_pos.y - drag_offset.y;

                rl.set_window_position(new_win_x as i32, new_win_y as i32);
            } else {
                is_dragging = false;
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLANK);
        d.draw_texture(&teto_img, 10, 10, Color::WHITE);
    }
}
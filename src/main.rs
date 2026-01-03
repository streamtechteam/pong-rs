mod window;
use raylib::{math::{Rectangle, Vector2}, window::{get_current_monitor, get_monitor_height, get_monitor_width}};
use window::Window;

mod game;
use game::Game;

use crate::game::UIProperties;

fn main() {
    println!("{}",get_current_monitor());

    let (mut rl, thread) = raylib::init()
        .title("PONG-rs")
        .width(0)
        .height(0)
        .fullscreen()
        // .resizable()
        .build();

    rl.set_window_size(rl.get_screen_width(), rl.get_screen_height());
    rl.set_target_fps(120);
    let mut game = Game {
        window: Window {
            width: rl.get_render_width(),
            height: rl.get_render_height(),
            height_slice: rl.get_render_height() / 12,
            width_slice: rl.get_render_width() / 12,
            center: Vector2 {
                x: (rl.get_render_width() / 12 * 6) as f32,
                y: (rl.get_render_height() / 12 * 6) as f32,
            },
        },
        ui_properties: UIProperties {
            menu_title_font_size: 150,
            menu_start_text_font_size: 100,
            game_title_font_size: 50,
            game_score_font_size: 100,
            debug_font_size: 20,
        },
        left_paddle: Rectangle {
            height: 250.,
            width: 50.,
            x: 0.,
            y: 0.,
        },
        right_paddle: Rectangle {
            height: 250.,
            width: 50.,
            x: 0.,
            y: 0.,
        },
        paddle_speed: 1000.,
        ball_radius: 20.,
        ball_pos: Vector2 { x: 0., y: 0. },
        ball_velocity: Vector2 { x: 0., y: 0. },
        ball_speed: 10.,
        ball_full_speed: 15.,
        ball_init_speed: 8.,
        left_score: 0,
        right_score: 0,
        win_score: 10,
        is_ended: false,
        winner: "".to_string(),
        is_running: true,
        show_debug: false,
        is_started: false,
    };

    game.start(&rl);
    game.game_loop(&mut rl, thread);
}

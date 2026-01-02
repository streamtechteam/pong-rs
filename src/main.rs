mod window;
use raylib::{
    RaylibThread,
    math::{Rectangle, Vector2},
};
use window::Window;

mod game;
use game::Game;

fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Simple Game")
        .width(1920)
        .height(1080)
        .fullscreen()
        .build();
    rl.set_target_fps(120);
    rl.trace_log(raylib::ffi::TraceLogLevel::LOG_ERROR, "test");
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
        left_score: 0,
        right_score: 0,
        is_running: true,
        show_debug: false,
        // ball_y:0,
        is_started: false,
    };

    game.start(&rl);
    game.game_loop(&mut rl, thread);
}

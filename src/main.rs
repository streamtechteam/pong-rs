

mod window;
use raylib::math::Vector2;
use window::Window;

mod game;
use game::Game;

fn main() {
    let (mut rl, thread) = raylib::init().title("Simple Game").width(1920).height(1080).resizable().build();
    rl.set_target_fps(480);
    let mut game = Game {
        window: Window {
            width: rl.get_render_width(),
            height: rl.get_render_height(),
            height_slice: rl.get_render_height() / 12,
            width_slice: rl.get_render_width() / 12,
            center: Vector2{
                x: (rl.get_render_width() / 12 * 6 ) as f32,
                y: (rl.get_render_height() / 12 * 6 ) as f32
            }
        },
        left_paddle_pos: Vector2 { x: 0., y: 0. },
        right_paddle_pos: Vector2 { x: 0., y: 0. },
        ball_pos: Vector2 { x: 0., y: 0. },
        ball_target_pos: Vector2 { x: 0., y: 0. },
        // ball_y:0,
        paddle_speed: 1000.,
    };

    game.start(&rl);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        game.update(&mut d);
    }
}

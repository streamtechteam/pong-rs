use crate::window::Window;
use rand::Rng;
use raylib::{
    RaylibHandle, RaylibThread,
    color::Color,
    ffi::{self},
    math::{Rectangle, Vector2},
    misc::AsF32,
    prelude::{RaylibDraw, RaylibDrawHandle},
    rgui::RaylibDrawGui,
};
#[derive(Default)]
pub struct Game {
    pub window: Window,
    pub left_paddle: Rectangle,
    pub right_paddle: Rectangle,
    pub paddle_speed: f32,
    pub ball_pos: Vector2,
    pub ball_radius: f32,
    pub ball_velocity: Vector2,
    pub ball_speed: f32,
    pub is_started: bool,
    pub left_score: i32,
    pub right_score: i32,
    pub show_debug: bool,
    pub is_running: bool,
}

impl Game {
    pub fn start(&mut self, rl: &RaylibHandle) {
        // println!("{}", self.window.height);
        self.left_paddle.y = (self.window.height_slice * 6) as f32;
        self.left_paddle.x = (self.window.width_slice * 1) as f32;
        // println!("{}",self.window.width);

        self.right_paddle.y = (self.window.height_slice * 6) as f32;
        self.right_paddle.x = (self.window.width_slice) as f32 * 12.;

        self.get_ball_direction(rl.get_frame_time());

        self.ball_pos.x = (self.window.width_slice * 6) as f32;
        self.ball_pos.y = (self.window.height_slice * 6) as f32;
    }
    pub fn game_loop(&mut self, rl: &mut RaylibHandle, thread: RaylibThread) {
        while !rl.window_should_close() && self.is_running {
            let mut d = rl.begin_drawing(&thread);
            self.update(&mut d);
        }
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) {
        // println!("{}", self.window.height);
        d.clear_background(Color::BLACK);
        self.update_window_props(d);

        if !self.is_started {
            if self.render_menu(d) {
                self.is_started = true;
            };
        } else {
            self.render_paddles(d);
            self.render_ball(d);
            self.render_debug(d);

            self.render_ui(d);
            self.move_paddles(d);
            self.move_ball(d);
        }
    }

    fn render_menu(&mut self, d: &mut RaylibDrawHandle) -> bool {
        let text_width = d.measure_text("PONG-rs!", 100);
        d.draw_text_pro(
            d.get_font_default(),
            "PONG-rs!",
            Vector2 {
                x: (self.window.width_slice * 6) as f32,
                y: (self.window.height_slice * 2) as f32,
            },
            Vector2 {
                x: (text_width / 2) as f32,
                y: 10.,
            },
            0.,
            100.,
            3.,
            Color::WHITE,
        );
        let start_button = d.gui_button(
            Rectangle {
                height: (self.window.height_slice * 1) as f32,
                width: (self.window.width_slice * 1) as f32,
                x: ((self.window.width_slice * 6) - (self.window.width_slice * 1 / 2)) as f32,
                y: ((self.window.height_slice * 6) - (self.window.height_slice * 1 / 2)) as f32,
            },
            "Start Game",
        );
        if d.gui_button(
            Rectangle {
                height: (self.window.height_slice * 1) as f32,
                width: (self.window.width_slice * 1) as f32,
                x: ((self.window.width_slice * 6) - (self.window.width_slice * 1 / 2)) as f32,
                y: ((self.window.height_slice * 7) - (self.window.height_slice * 1 / 2)) as f32,
            },
            "Exit Game",
        ) {
            self.is_running = false;
        }
        start_button
    }

    fn render_ui(&mut self, d: &mut RaylibDrawHandle) {
        // let text_width = d.measure_text(self.score.to_string().as_str(), 20);
        // d.draw_text_pro(
        //     d.get_font_default(),
        //     format!("Score: {}", self.score).as_str(),
        //     Vector2 {
        //         x: self.window.width_slice as f32 * 0.5,
        //         y: 20.,
        //     },
        //     Vector2 {
        //         x: (text_width / 2) as f32,
        //         y: 10.,
        //     },
        //     0.,
        //     20.,
        //     3.,
        //     Color::WHITE,
        // );
        let mut ys: Vec<f32> = vec![];
        for num in 2..25{
            ys.push((d.get_render_height() / 24 * num) as f32);
        }
        for num in ys {
            d.draw_rectangle_pro(Rectangle{
                height: 10.,
                width: 5.,
                x : (self.window.width_slice * 6) as f32,
                y : num,
            }, Vector2{
                x: 2.5,
                y: 5.,
            }, 0., Color::WHITE);
        }
        let text_width = d.measure_text(self.left_score.to_string().as_str(), 100);
        d.draw_text_pro(
            d.get_font_default(),
            format!("{}", self.left_score).as_str(),
            Vector2 {
                x: (self.window.width_slice * 3) as f32,
                y: (self.window.height_slice * 6) as f32,
            },
            Vector2 {
                x: (text_width / 2) as f32,
                y: 10.,
            },
            0.,
            100.,
            3.,
            Color::WHITE,
        );
        let text_width = d.measure_text(self.right_score.to_string().as_str(), 100);
        d.draw_text_pro(
            d.get_font_default(),
            format!("{}", self.right_score).as_str(),
            Vector2 {
                x: (self.window.width_slice * 9) as f32,
                y: (self.window.height_slice * 6) as f32,
            },
            Vector2 {
                x: (text_width / 2) as f32,
                y: 10.,
            },
            0.,
            100.,
            3.,
            Color::WHITE,
        );

        let text_width = d.measure_text("PONG-rs!", 50);
        d.draw_text_pro(
            d.get_font_default(),
            "PONG-rs!",
            Vector2 {
                x: (self.window.width_slice * 6) as f32,
                y: 20.,
            },
            Vector2 {
                x: (text_width / 2) as f32,
                y: 10.,
            },
            0.,
            50.,
            3.,
            Color::WHITE,
        );
    }

    fn render_debug(&mut self, d: &mut RaylibDrawHandle) {
        if d.is_key_pressed(ffi::KeyboardKey::KEY_SEMICOLON) {
            self.show_debug = !self.show_debug;
        }
        if (self.show_debug) {
            d.draw_fps(40, 40);
        }
    }

    fn render_paddles(&mut self, d: &mut RaylibDrawHandle) {
        // self.right_paddle_pos.y = (self.window.height_slice * 6) as f32;
        self.left_paddle.x = (self.window.width_slice) as f32 * 0.5;

        self.right_paddle.x = (self.window.width_slice) as f32 * 11.5;

        d.draw_rectangle_pro(
            self.left_paddle,
            Vector2 {
                x: self.left_paddle.width / 2.,
                y: self.left_paddle.height / 2.,
            },
            0.,
            Color::WHITE,
        );
        d.draw_rectangle_pro(
            self.right_paddle,
            Vector2 {
                x: self.right_paddle.width / 2.,
                y: self.right_paddle.height / 2.,
            },
            0.,
            Color::WHITE,
        );
    }
    fn move_paddles(&mut self, d: &mut RaylibDrawHandle) {
        // let window_height = &d.get_render_height();
        // let window_width = &d.get_render_width();

        if self.left_paddle.y > (self.window.height_slice) as f32  * 2.{
            if d.is_key_down(ffi::KeyboardKey::KEY_W) {
                self.left_paddle.y -= d.get_frame_time() * self.paddle_speed;
            }
        }
        if self.left_paddle.y < (self.window.height_slice) as f32  * 10.{
            if d.is_key_down(ffi::KeyboardKey::KEY_S) {
                self.left_paddle.y += d.get_frame_time() * self.paddle_speed;
            }
        }

        // println!("{}",self.right_paddle_pos.y);

        if d.is_key_down(ffi::KeyboardKey::KEY_UP) {
            if self.right_paddle.y > (self.window.height_slice * 2) as f32 {
                self.right_paddle.y -= d.get_frame_time() * self.paddle_speed;
            }
        }
        if d.is_key_down(ffi::KeyboardKey::KEY_DOWN) {
            if self.right_paddle.y <= (self.window.height_slice * 10) as f32 {
                self.right_paddle.y += d.get_frame_time() * self.paddle_speed;
            }
        }
    }
    fn render_ball(&mut self, d: &mut RaylibDrawHandle) {
        // d.draw_circle(self.ball_pos.x as i32, self.ball_pos.y as i32, 20., Color::WHITE);
        d.draw_circle_v(self.ball_pos, self.ball_radius, Color::WHITE);
    }
    fn move_ball(&mut self, d: &mut RaylibDrawHandle) {
        let left_paddle_rect = Rectangle {
            height: self.left_paddle.height,
            width: self.left_paddle.width,
            x: self.left_paddle.x - self.left_paddle.width / 2.,
            y: self.left_paddle.y - self.left_paddle.height / 2.,
        };
        let right_paddle_rect = Rectangle {
            height: self.right_paddle.height,
            width: self.right_paddle.width,
            x: self.right_paddle.x - self.right_paddle.width / 2.,
            y: self.right_paddle.y - self.right_paddle.height / 2.,
        };
        if left_paddle_rect.check_collision_circle_rec(self.ball_pos, self.ball_radius) {
            self.ball_velocity.x = self.ball_velocity.x * -1.0;
        }
        if right_paddle_rect.check_collision_circle_rec(self.ball_pos, self.ball_radius) {
            self.ball_velocity.x = self.ball_velocity.x * -1.0;
            self.ball_velocity.y = self.ball_velocity.y * -1.0;
        }
        if self.ball_pos.y <= 0. + self.ball_radius / 2.
            || self.ball_pos.y >= d.get_render_height().as_f32() - self.ball_radius / 2.
        {
            self.ball_velocity.y = self.ball_velocity.y * -1.0;
        }
        if self.ball_pos.x <= 0. {
            self.right_score += 1;
            self.ball_pos = self.window.center;
        }
        if self.ball_pos.x >= d.get_render_width().as_f32() {
            self.left_score += 1;
            self.ball_pos = self.window.center;

            // thread::sleep(Duration::from_secs(1));
        }

        self.ball_pos += self.ball_velocity;
    }

    fn get_ball_direction(&mut self, dt: f32) {
        let is_left = rand::rng().random_bool(0.5);
        // println!("{}", dt);
        if is_left {
            self.ball_velocity.x = (self.ball_speed * -0.8);
            self.ball_velocity.y = (self.ball_speed * -0.2);
            // self.ball_velocity = Vector2 { x: self.left_paddle.x, y: self.left_paddle.y };
        } else {
            self.ball_velocity.x = (self.ball_speed * 0.8);
            self.ball_velocity.y = (self.ball_speed * 0.2);

            // self.ball_velocity = Vector2 { x: self.right_paddle.x, y: self.right_paddle.y };
        }
    }

    fn update_window_props(&mut self, d: &mut RaylibDrawHandle) {
        self.window.height = d.get_render_height();
        self.window.width = d.get_render_width();
        self.window.height_slice = self.window.height / 12;
        self.window.width_slice = self.window.width / 12;
        self.window.center = Vector2 {
            x: (self.window.width_slice * 6) as f32,
            y: (self.window.height_slice * 6) as f32,
        }
    }
}

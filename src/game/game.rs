use crate::{game::UIProperties, window::Window};
use rand::Rng;
use raylib::{
    RaylibHandle, RaylibThread,
    color::Color,
    ffi::{self, KeyboardKey},
    math::{Rectangle, Vector2},
    misc::AsF32,
    prelude::{RaylibDraw, RaylibDrawHandle}, rgui::RaylibDrawGui,
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
    pub ball_full_speed: f32,
    pub ball_init_speed: f32,
    pub is_started: bool,
    pub is_running: bool,
    pub is_ended: bool,
    pub left_score: i32,
    pub right_score: i32,
    pub win_score: i32,
    pub winner: String,
    pub show_debug: bool,
    pub ui_properties: UIProperties,
}

impl Game {
    pub fn start(&mut self, rl: &RaylibHandle) {
        // println!("{}", self.window.height);
        self.left_paddle.y = (self.window.height_slice * 6) as f32;
        self.left_paddle.x = (self.window.width_slice * 1) as f32;
        // println!("{}",self.window.width);

        self.right_paddle.y = (self.window.height_slice * 6) as f32;
        self.right_paddle.x = (self.window.width_slice) as f32 * 12.;

        self.ball_pos.x = (self.window.width_slice * 6) as f32;
        self.ball_pos.y = (self.window.height_slice * 6) as f32;

        self.ball_speed = self.ball_init_speed;
        self.get_ball_direction();
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

        // println!("{}",self.ball_speed);
        // self.update_ball_speed();
        if !self.is_ended {
            if !self.is_started {
                if self.draw_menu_ui(d) {
                    self.is_started = true;
                };
            } else {
                if d.is_key_pressed(KeyboardKey::KEY_R) {
                    self.reset_game(d);
                }
                self.render_paddles(d);
                self.render_ball(d);
                self.draw_debug_ui(d);
                self.handle_score(d);
                self.draw_hud_ui(d);
                self.handle_paddles_movement(d);
                self.handle_ball_movement(d);
            }
        } else {
            self.draw_end_game_ui(d);
        }
    }

    fn draw_menu_ui(&mut self, d: &mut RaylibDrawHandle) -> bool {
        let text_width = d.measure_text("PONG-rs!", self.ui_properties.menu_title_font_size);
        d.draw_text_pro(
            d.get_font_default(),
            "PONG-rs!",
            Vector2 {
                x: (self.window.width_slice) as f32 * 6.2,
                y: (self.window.height_slice * 2) as f32,
            },
            Vector2 {
                x: (text_width / 2) as f32,
                y: 10.,
            },
            // Vector2{
            //   x:0.,
            //   y:0.
            // },
            0.,
            self.ui_properties.menu_title_font_size.as_f32(),
            3.,
            Color::WHITE,
        );
        let text_width = d.measure_text(
            "> Press any key to start! <",
            self.ui_properties.menu_start_text_font_size,
        );
        d.draw_text_pro(
            d.get_font_default(),
            "> Press any key to start! <",
            Vector2 {
                x: (self.window.width_slice) as f32 * 6.6,
                y: (self.window.height_slice * 7) as f32,
            },
            Vector2 {
                x: (text_width / 2) as f32,
                y: 10.,
            },
            0.,
            self.ui_properties.menu_start_text_font_size.as_f32(),
            3.,
            Color::WHITE,
        );
        d.get_key_pressed().is_some()
    }

    fn draw_hud_ui(&mut self, d: &mut RaylibDrawHandle) {
        // rendering center_lines
        {
            let mut lines: Vec<f32> = vec![];
            for num in 2..25 {
                lines.push((d.get_render_height() / 24 * num) as f32);
            }
            for line in lines {
                d.draw_rectangle_pro(
                    Rectangle {
                        height: 10.,
                        width: 5.,
                        x: (self.window.width_slice * 6) as f32,
                        y: line,
                    },
                    Vector2 { x: 2.5, y: 5. },
                    0.,
                    Color::WHITE,
                );
            }
        }
        // rendering up and down lines
        {
            d.draw_line_ex(
                Vector2 { x: 0., y: 0. },
                Vector2 {
                    x: d.get_render_width().as_f32(),
                    y: 0.,
                },
                10.,
                Color::WHITE,
            );
            d.draw_line_ex(
                Vector2 {
                    x: 0.,
                    y: d.get_render_height().as_f32(),
                },
                Vector2 {
                    x: d.get_render_width().as_f32(),
                    y: d.get_render_height().as_f32(),
                },
                10.,
                Color::WHITE,
            );
        }
        // rendering title
        {
            let text_width = d.measure_text("PONG-rs!", self.ui_properties.game_title_font_size);
            d.draw_text_pro(
                d.get_font_default(),
                "PONG-rs!",
                Vector2 {
                    x: (self.window.width_slice * 6) as f32,
                    y: 30.,
                },
                Vector2 {
                    x: (text_width / 2) as f32,
                    y: 10.,
                },
                0.,
                self.ui_properties.game_title_font_size as f32,
                3.,
                Color::WHITE,
            );
        }
        // rendering scores
        {
            let text_width = d.measure_text(
                self.left_score.to_string().as_str(),
                self.ui_properties.game_score_font_size,
            );
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
                self.ui_properties.game_score_font_size.as_f32(),
                3.,
                Color::WHITE,
            );
            let text_width = d.measure_text(
                self.right_score.to_string().as_str(),
                self.ui_properties.game_score_font_size,
            );
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
                self.ui_properties.game_score_font_size.as_f32(),
                3.,
                Color::WHITE,
            );
        }
    }

    fn draw_debug_ui(&mut self, d: &mut RaylibDrawHandle) {
        if d.is_key_pressed(ffi::KeyboardKey::KEY_SEMICOLON) {
            self.show_debug = !self.show_debug;
        }
        if self.show_debug {
            d.draw_fps(40, 40);
            d.draw_text(
                format!("ball_velocity: {:?}", self.ball_velocity).as_str(),
                40,
                60,
                self.ui_properties.debug_font_size,
                Color::RED,
            );
        }
    }
    fn draw_end_game_ui(&mut self, d: &mut RaylibDrawHandle) {
        let text_width = d.measure_text(
            format!("> {} is the winner ! <", self.winner).as_str(),
            self.ui_properties.menu_start_text_font_size,
        );
        d.draw_text_pro(
            d.get_font_default(),
            format!("> {} is the winner ! <", self.winner).as_str(),
            Vector2 {
                x: (self.window.width_slice) as f32 * 6.6,
                y: (self.window.height_slice * 7) as f32,
            },
            Vector2 {
                x: (text_width / 2) as f32,
                y: 10.,
            },
            0.,
            self.ui_properties.menu_start_text_font_size.as_f32(),
            3.,
            Color::WHITE,
        );
        let text_width = d.measure_text("Press Space to restart", 20);
        d.draw_text_pro(
            d.get_font_default(),
            "Press Space to restart",
            Vector2 {
                x: (self.window.width_slice) as f32 * 6.,
                y: (self.window.height_slice) as f32 * 8.2,
            },
            Vector2 {
                x: (text_width / 2) as f32,
                y: 10.,
            },
            0.,
            20.,
            3.,
            Color::WHITE,
        );

        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            self.reset_game(d);
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
    fn handle_paddles_movement(&mut self, d: &mut RaylibDrawHandle) {
        // let window_height = &d.get_render_height();
        // let window_width = &d.get_render_width();

        if self.left_paddle.y > (self.window.height_slice) as f32 * 1.6 {
            if d.is_key_down(ffi::KeyboardKey::KEY_W) {
                self.left_paddle.y -= d.get_frame_time() * self.paddle_speed;
            }
        }
        if self.left_paddle.y < (self.window.height_slice) as f32 * 10.4 {
            if d.is_key_down(ffi::KeyboardKey::KEY_S) {
                self.left_paddle.y += d.get_frame_time() * self.paddle_speed;
            }
        }

        if d.is_key_down(ffi::KeyboardKey::KEY_UP) {
            if self.right_paddle.y > (self.window.height_slice) as f32 * 1.6 {
                self.right_paddle.y -= d.get_frame_time() * self.paddle_speed;
            }
        }
        if d.is_key_down(ffi::KeyboardKey::KEY_DOWN) {
            if self.right_paddle.y <= (self.window.height_slice) as f32 * 10.4 {
                self.right_paddle.y += d.get_frame_time() * self.paddle_speed;
            }
        }
    }

    fn reset_paddles(&mut self, d: &mut RaylibDrawHandle) {
        self.left_paddle.y = (self.window.height_slice) as f32 * 6.0;
        self.right_paddle.y = (self.window.height_slice) as f32 * 6.0;
    }

    fn render_ball(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.ball_pos, self.ball_radius, Color::WHITE);
    }
    fn handle_ball_movement(&mut self, d: &mut RaylibDrawHandle) {
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
            self.ball_speed = self.ball_full_speed;
            self.update_ball_speed();

            self.ball_velocity.x = self.ball_velocity.x * -1.0;
        }
        if right_paddle_rect.check_collision_circle_rec(self.ball_pos, self.ball_radius) {
            self.ball_speed = self.ball_full_speed;
            self.update_ball_speed();

            self.ball_velocity.x = self.ball_velocity.x * -1.0;
            // self.ball_velocity.y = self.ball_velocity.y * -1.0;
        }
        if self.ball_pos.y <= 0. + self.ball_radius / 2.
            || self.ball_pos.y >= d.get_render_height().as_f32() - self.ball_radius as f32
        {
            self.ball_speed = self.ball_full_speed;
            self.update_ball_speed();
            self.ball_velocity.y = self.ball_velocity.y * -1.0;
        }
        if self.ball_pos.x <= 0. {
            self.right_score += 1;
            self.reset_ball(d);
        }
        if self.ball_pos.x >= d.get_render_width().as_f32() {
            self.left_score += 1;
            self.reset_ball(d);
        }

        self.ball_pos += self.ball_velocity;
    }

    fn reset_ball(&mut self, d: &mut RaylibDrawHandle) {
        self.ball_pos = self.window.center;
        self.ball_speed = self.ball_init_speed;
        self.update_ball_speed();
    }

    fn get_ball_direction(&mut self) {
        let is_left = rand::rng().random_bool(0.5);
        if is_left {
            self.ball_velocity.x = self.ball_speed * -0.8;
            self.ball_velocity.y = self.ball_speed * -0.2;
            // self.ball_velocity = Vector2 { x: self.left_paddle.x, y: self.left_paddle.y };
        } else {
            self.ball_velocity.x = self.ball_speed * 0.8;
            self.ball_velocity.y = self.ball_speed * 0.2;

            // self.ball_velocity = Vector2 { x: self.right_paddle.x, y: self.right_paddle.y };
        }
    }
    fn update_ball_speed(&mut self) {
        // if self.ball_velocity.x.is_sign_positive(){
        //     // self.ball_velocity.x = self.ball_speed *
        // }

        self.ball_velocity.x *=
            self.ball_speed / (self.ball_velocity.x.abs() + self.ball_velocity.y.abs());
        self.ball_velocity.y *=
            self.ball_speed / (self.ball_velocity.x.abs() + self.ball_velocity.y.abs());
    }

    fn reset_game(&mut self, d: &mut RaylibDrawHandle) {
        self.is_ended = false;
        self.is_started = false;
        self.winner = "".to_string();
        self.left_score = 0;
        self.right_score = 0;
        self.get_ball_direction();
        self.reset_ball(d);
        self.reset_paddles(d);
    }
    fn handle_score(&mut self, d: &mut RaylibDrawHandle) {
        if self.left_score >= self.win_score {
            self.is_ended = true;
            self.winner = "Left Player".to_string();
        } else if self.right_score >= self.win_score {
            self.is_ended = true;
            self.winner = "Right Player".to_string();
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

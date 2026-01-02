use crate::window::Window;
use rand::Rng;
use raylib::{
    RaylibHandle,
    color::Color,
    ffi::{self, Font},
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
};

pub struct Game {
    pub window: Window,
    pub left_paddle_pos: Vector2,
    pub right_paddle_pos: Vector2,
    pub paddle_speed : f32,
    pub ball_pos: Vector2,
    pub ball_target_pos: Vector2,
    // d: RaylibDrawHandle,
}

impl Game {
    pub fn start(&mut self, rl: &RaylibHandle) {
        println!("{}", self.window.height);
        self.left_paddle_pos.y = (self.window.height_slice * 6) as f32;
        // println!("{}",self.window.width);

        self.right_paddle_pos.y = (self.window.height_slice * 6) as f32;

        self.ball_pos.x = (self.window.width_slice * 6) as f32;
        self.ball_pos.y = (self.window.height_slice * 6) as f32;
    }
    pub fn update(&mut self, d: &mut RaylibDrawHandle) {
        println!("{}", self.window.height);

        d.clear_background(Color::BLACK);
        self.render_paddles(d);
        self.move_paddles(d);
        self.render_ball(d);
        self.move_ball(d);
        self.update_window_props(d);
        self.render_debug(d);
    }
    fn render_debug(&self ,d: &mut RaylibDrawHandle) {
        let text_width = d.measure_text("Simple Game", 20);
        d.draw_text_pro(
            d.get_font_default(),
            "Simple Game",
            Vector2{
                x:(self.window.width_slice * 6) as f32,
                y: 20.
            },
            Vector2{
                x:(text_width / 2) as f32,
                y: 10.
            },
            0.,
            20.,
            3.,
            Color::WHITE,
        );
        d.draw_fps(40, 40);
    }

    fn render_paddles(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_pro(
            Rectangle {
                x: self.left_paddle_pos.x,
                y: self.left_paddle_pos.y,
                height: 250.,
                width: 50.,
            },
            Vector2 { x: 25., y: 125. },
            0.,
            Color::WHITE,
        );
        d.draw_rectangle_pro(
            Rectangle {
                x: self.right_paddle_pos.x ,
                y: self.right_paddle_pos.y ,
                height: 250.,
                width: 50.,
            },
            Vector2 { x: 25., y: 125. },
            0.,
            Color::WHITE,
        );
    }
    fn move_paddles(&mut self, d: &mut RaylibDrawHandle) {
        // let window_height = &d.get_render_height();
        // let window_width = &d.get_render_width();

        if self.left_paddle_pos.y > (self.window.height_slice * 2) as f32 {
            if (d.is_key_down(ffi::KeyboardKey::KEY_UP)) {
                self.left_paddle_pos.y -= d.get_frame_time() * self.paddle_speed;
            }
        }
        if self.left_paddle_pos.y < (self.window.height_slice * 10) as f32 {
            if (d.is_key_down(ffi::KeyboardKey::KEY_DOWN)) {
                self.left_paddle_pos.y += d.get_frame_time() * self.paddle_speed;
            }
        }

        // println!("{}",self.right_paddle_pos.y);

        if (d.is_key_down(ffi::KeyboardKey::KEY_W)) {
            if self.right_paddle_pos.y > (self.window.height_slice * 2) as f32 {
                self.right_paddle_pos.y -= d.get_frame_time() * self.paddle_speed;
            }
        }
        if (d.is_key_down(ffi::KeyboardKey::KEY_S)) {
            if self.right_paddle_pos.y <= (self.window.height_slice * 10) as f32 {
                self.right_paddle_pos.y += d.get_frame_time() * self.paddle_speed;
            }
        }
    }



    fn render_ball(&mut self, d: &mut RaylibDrawHandle){
        d.draw_circle(self.ball_pos.x as i32, self.ball_pos.y as i32, 20., Color::WHITE);
    }
    fn move_ball(&mut self, d: &mut RaylibDrawHandle){
        // self.ball_pos.x = (self.window.width_slice * 6) as f32;
        // self.ball_pos.y = (self.window.height_slice * 6) as f32;


    }


    fn get_ball_direction(&mut self , d: &mut RaylibDrawHandle){
        let is_left = rand::rng().random_bool(50.);
        if is_left{
            self.ball_target_pos = self.left_paddle_pos
        }
        else {
            self.ball_target_pos = self.right_paddle_pos
        }
    }

    fn update_window_props(&mut self, d: &mut RaylibDrawHandle) {
        self.window.height = d.get_render_height();
        self.window.width = d.get_render_width();
        self.window.height_slice = self.window.height / 12;
        self.window.width_slice = self.window.width / 12;
        self.window.center = Vector2{
            x: (self.window.width_slice * 6 ) as f32,
            y: (self.window.height_slice * 6 ) as f32
        }
    }
}

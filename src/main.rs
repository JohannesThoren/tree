extern crate sdl2;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    sys,
    video::Window,
};

const SCREENW: u32 = 1000;
const SCREENH: u32 = 800;
const DEPTH: u32 = 11 ;

struct Tree {
    line_points_list: Vec<(i32, i32, i32, i32)>,
    ang: f64,
}

impl Tree {
    fn new() -> Self {
        Tree {
            line_points_list: Vec::new(),
            ang: 20.0,
        }
    }
    pub fn generate_tree_line_points(&mut self, x1: f64, y1: f64, angle: f64, depth: u32) {
        let x2 = x1 + angle.to_radians().sin() * depth as f64 * 10.0;
        let y2 = y1 - angle.to_radians().cos() * depth as f64 * 10.0;

        &self
            .line_points_list
            .push((x1 as i32, y1 as i32, x2 as i32, y2 as i32));

        if depth > 0 {
            self.generate_tree_line_points(x2, y2, angle + &self.ang, depth - 1);
            self.generate_tree_line_points(x2, y2, angle - &self.ang, depth - 1);
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for line_points in &self.line_points_list {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.draw_line(
                Point::new(line_points.0, line_points.1),
                Point::new(line_points.2, line_points.3),
            );
        }

        canvas.present();
    }
}                       

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let window = video_subsystem
        .window("tree", SCREENW, SCREENH)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut running = true;
    let mut tree = Tree::new();
    tree.generate_tree_line_points((SCREENW / 2).into(), (SCREENH - 100).into(), 0.0, DEPTH);
    while running {
        tree.draw(&mut canvas);
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { timestamp, window_id, keycode, scancode, keymod,  repeat} => {
                    match keycode.unwrap() {
                        Keycode::Up => {
                            tree.line_points_list = Vec::new();
                            tree.ang += 1.0;
                            tree.generate_tree_line_points((SCREENW / 2).into(), (SCREENH - 100).into(), 0.0, DEPTH);
                        },
                        Keycode::Down => {
                            tree.line_points_list = Vec::new();
                            tree.ang -= 1.0;
                            tree.generate_tree_line_points((SCREENW / 2).into(), (SCREENH - 100).into(), 0.0, DEPTH);                        },
                        _ => {}
                    }
                }

                Event::Quit { .. } => {running = false;}
                _ => {}
            }
        }
    }
}

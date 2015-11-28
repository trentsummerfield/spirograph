use sim_interface::Bitmap;
use sim_interface::Input;
use sim_interface::SimState;

use std::any::Any;
use std::f32::consts::PI;
use std::ops;

struct State {
    initialized: bool,
    frame_num: u64,
}

impl SimState for State {
    fn as_mut_any(&mut self) -> &mut Any {
        self
    }
}

impl Bitmap {
    fn set_pixel(&mut self, x: isize, y: isize) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return;
        }
        let index = x * 4 + y * self.pitch;
        self.memory[index + 0] = 242;
        self.memory[index + 1] = 210;
        self.memory[index + 2] = 52;
        self.memory[index + 3] = 0;
    }

    fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = x * 4 + y * self.pitch;
                self.memory[index + 0] = 53;
                self.memory[index + 1] = 51;
                self.memory[index + 2] = 44;
                self.memory[index + 3] = 0;
            }
        }
    }
}

pub fn update_and_render(bitmap: &mut Bitmap, input: &Input, sim_state: Option<Box<SimState>>) -> Box<SimState> {
    bitmap.clear();
    let mut sim_state = sim_state.unwrap_or(Box::new(State{initialized: false, frame_num: 0}));
    {
        let state = sim_state.as_mut_any().downcast_mut().unwrap();
        draw(bitmap, input, state);
        state.frame_num += 1;
    }
    return sim_state;
}

#[derive(Copy, Clone)]
struct V2 {
    x: f32,
    y: f32,
}

impl V2 {
    fn new(x: f32, y: f32) -> V2 {
        V2 { x: x, y: y }
    }

    fn scale(self, s: f32) -> V2 {
        V2::new(self.x * s, self.y * s)
    }

    fn add(self, o: V2) -> V2 {
        V2::new(self.x + o.x, self.y + o.y)
    }
}

fn circle(t: f32, r: f32) -> V2 {
    return V2::new(r * t.cos(), r * t.sin());
}

fn spiro(t: f32, big_radius: f32, little_radius: f32, l: f32) -> V2 {
    let k = little_radius / big_radius;
    let one_k = 1.0 - k;
    let lk = l * k;
    let t2 = (one_k / k) * t;
    let x = big_radius * (one_k*t.cos() + lk*t2.cos());
    let y = big_radius * (one_k*t.sin() - lk*t2.sin());
    return V2::new(x, y);
}

fn draw(bitmap: &mut Bitmap, input: &Input, sim_state: &mut State) {
    let dim = V2::new(bitmap.width as f32, bitmap.height as f32);
    let offset = dim.scale(0.5);
    let period = (sim_state.frame_num as f32 / 20.0);
    draw_parametric(bitmap, period, |t| spiro(t, 220.0, 65.0, 0.8).add(dim.scale(0.5)));
}

fn draw_parametric<F>(bitmap: &mut Bitmap, period: f32, parametric_func: F) -> ()
    where F : Fn(f32) -> V2 {
    let mut p = parametric_func(0.0);
    let mut t = 0.0;
    loop {
        let q = parametric_func(t);
        draw_line(bitmap, p.x, p.y, q.x, q.y);
        p = q;
        t += 0.01;
        if t > period {
            break;
        }
    }
}

fn draw_line(bitmap: &mut Bitmap, x1: f32, y1: f32, x2: f32, y2: f32) {
    // Bresenham's line algorithm
    let mut x1 = x1;
    let mut x2 = x2;
    let mut y1 = y1;
    let mut y2 = y2;
    let steep = (y2 - y1).abs() > (x2 - x1).abs();
    let mut tmp;
    if steep {
        tmp = x1;
        x1 = y1;
        y1 = tmp;
        tmp = x2;
        x2 = y2;
        y2 = tmp;
    }

    if x1 > x2 {
        tmp = x1;
        x1 = x2;
        x2 = tmp;
        tmp = y1;
        y1 = y2;
        y2 = tmp;
    }

    let dx = x2 - x1;
    let dy = (y2 - y1).abs();

    let mut error = dx / 2.0;
    let ystep: isize = if y1 < y2 { 1 } else { -1 };
    let mut y = y1.trunc() as isize;

    let max_x = x2.trunc() as isize;

    let x = x1.trunc() as isize;
    for x in x..max_x
    {
        if steep {
            bitmap.set_pixel(y, x);
        } else {
            bitmap.set_pixel(x, y);
        }

        error -= dy;
        if error < 0.0 {
            y += ystep;
            error += dx;
        }
    }
}

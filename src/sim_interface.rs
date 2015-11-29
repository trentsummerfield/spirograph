use std::any::Any;

pub trait SimState {
    fn as_mut_any(&mut self) -> &mut Any;
}

pub struct Bitmap {
    pub memory: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
}

#[derive(Default)]
pub struct Button {
    pub half_transition_count: usize,
    pub ended_down: bool,
}

#[derive(Default)]
pub struct Controller {
    pub active: usize,
    pub up: Button,
    pub down: Button,
    pub left: Button,
    pub right: Button,
    pub left_shoulder: Button,
    pub right_shoulder: Button,
    pub a: Button,
    pub b: Button,
    pub x: Button,
    pub y: Button,
}

pub struct Input {
    pub dt: f32,
    pub keyboard: Controller,
    pub command_line: Vec<String>
}

impl Input {
    pub fn new(command_line: Vec<String>) -> Input {
        return Input{
            dt: 1.0 / 60.0,
            keyboard: Controller {active: 1, ..Default::default()},
            command_line: command_line,
        };
    }
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Bitmap {
        Bitmap {
            memory: vec![0; width * height * 4],
            width: width,
            height: height,
            pitch: width * 4,
        }
    }
}

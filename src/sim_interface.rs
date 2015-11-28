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
pub struct button_state {
    pub half_transition_count: usize,
    pub ended_down: bool,
}

#[derive(Default)]
pub struct Controller {
    pub active: usize,
    pub up: button_state,
    pub down: button_state,
    pub left: button_state,
    pub right: button_state,
    pub left_shoulder: button_state,
    pub right_shoulder: button_state,
    pub a: button_state,
    pub b: button_state,
    pub x: button_state,
    pub y: button_state,
}

pub struct Input {
    pub dt: f32,
    pub keyboard: Controller,
}

impl Input {
    pub fn new() -> Input {
        return Input{
            dt: 1.0 / 60.0,
            keyboard: Controller {active: 1, ..Default::default()}
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

use bevy::asset::Assets;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use rand::random;
use rand_derive::Rand;
use crate::game::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::game::shapes::{BACK_L_SHAPE, BACK_Z_SHAPE, L_SHAPE, LINE, SQUARE, Z_SHAPE};
use crate::game::tetris_board::{Colors, TetrisBoard};

#[derive(Rand)]
enum Tetrominos {
    LShape,
    BackLShape,
    Line,
    Square,
    ZShape,
    BackZShape
}

impl Tetrominos {
    pub fn get_shape(&self) -> &'static [[[bool; 4]; 4]; 4] {
        match &self {
            Tetrominos::LShape => &L_SHAPE,
            Tetrominos::BackLShape => &BACK_L_SHAPE,
            Tetrominos::Line => &LINE,
            Tetrominos::Square => &SQUARE,
            Tetrominos::ZShape => &Z_SHAPE,
            Tetrominos::BackZShape => &BACK_Z_SHAPE
        }
    }
}

#[derive(Resource)]
pub struct Ticker {
    last: f32,
    interval: f32
}

impl Ticker {
    pub fn new(time: &Time, interval: f32) -> Ticker {
        Ticker {
            last: time.elapsed_seconds(),
            interval
        }
    }

    pub fn ticks(&mut self, time: &Time) -> usize {
        let ticks = ((time.elapsed_seconds() - self.last) / self.interval) as usize;

        self.last += self.interval * ticks as f32;

        ticks
    }
}

#[derive(Resource)]
pub struct TetrisLogic {
    x: usize,
    y: usize,
    current_shape: Tetrominos
}


impl TetrisLogic {
    pub fn new() -> TetrisLogic {
        TetrisLogic {
            x: 0,
            y: 0,
            current_shape: random()
        }
    }

    pub fn update(&mut self, board: &mut TetrisBoard, keyboard: &ButtonInput<KeyCode>, materials: &mut Assets<ColorMaterial>) {
        let old_pos = (self.x, self.y);

        if keyboard.just_pressed(KeyCode::ArrowRight) && self.x < BOARD_WIDTH - 1 {
            self.x += 1;
        }
        if keyboard.just_pressed(KeyCode::ArrowLeft) && self.x > 0 {
            self.x -= 1;
        }
        if keyboard.just_pressed(KeyCode::ArrowUp) && self.y < BOARD_HEIGHT - 1 {
            self.y += 1;
        }
        if keyboard.just_pressed(KeyCode::ArrowDown) && self.y > 0 {
            self.y -= 1;
        }

        board.set_cell_colour(old_pos.0, old_pos.1, Colors::Empty, materials);
        board.set_cell_colour(self.x, self.y, Colors::Red, materials);
    }

    pub fn tick(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
}

pub fn tetris_logic_setup(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(TetrisLogic::new());
    commands.insert_resource(Ticker::new(&time, 1.0 / 2.0));
}

pub fn tetris_logic_update(
    mut logic: ResMut<TetrisLogic>,
    mut board: ResMut<TetrisBoard>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ticker: ResMut<Ticker>,
    time: Res<Time>
) {
    for _ in 0..ticker.as_mut().ticks(&time) {
        logic.as_mut().tick();
    }

    logic.as_mut().update(board.as_mut(), &keyboard, materials.as_mut());
}

pub fn tetris_logic_shutdown(mut commands: Commands) {
    commands.remove_resource::<TetrisLogic>();
    commands.remove_resource::<Ticker>()
}
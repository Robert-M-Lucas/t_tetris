use bevy::asset::Assets;
use bevy::input::ButtonInput;
use bevy::prelude::{ColorMaterial, Commands, KeyCode, Res, ResMut, Resource};
use crate::game::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::game::tetris_board::{Colors, TetrisBoard};

#[derive(Resource)]
pub struct TetrisLogic {
    x: usize,
    y: usize
}

impl TetrisLogic {
    pub fn new() -> TetrisLogic {
        TetrisLogic { x: 0, y: 0 }
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
}

pub fn tetris_logic_setup(mut commands: Commands) {
    commands.insert_resource(TetrisLogic::new());
}

pub fn tetris_logic_update(
    mut logic: ResMut<TetrisLogic>,
    mut board: ResMut<TetrisBoard>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    logic.as_mut().update(board.as_mut(), &keyboard, materials.as_mut());
}

pub fn tetris_logic_shutdown(mut commands: Commands) {
    commands.remove_resource::<TetrisLogic>();
}
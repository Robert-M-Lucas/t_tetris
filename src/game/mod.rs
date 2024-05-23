use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::sprite::Material2d;
use rand::random;
use tetris_board::{Colors, TetrisBoard};

use crate::game::ui_setup::{ui_resize_handler, ui_setup};
use crate::GameState;
use crate::util::despawn_screen;

mod ui_setup;
mod tetris_board;
mod tetris_logic;
mod shapes;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 24;
pub const BOARD_WIDTH_F: f32 = BOARD_WIDTH as f32;
pub const BOARD_HEIGHT_F: f32 = BOARD_HEIGHT as f32;

pub const RATIO: f32 = BOARD_WIDTH_F / BOARD_HEIGHT_F;

#[derive(Component)]
pub struct OnGameScreen;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Game),
                (ui_setup, tetris_board::tetris_board_setup, tetris_logic::tetris_logic_setup)
            )
            .add_systems(
                Update,
                (ui_resize_handler, tetris_board::cell_resize_handler, tetris_logic::tetris_logic_update)
                    .run_if(in_state(GameState::Game))
            )
            .add_systems(
                OnExit(GameState::Game),
                (despawn_screen::<OnGameScreen>, tetris_board::tetris_board_shutdown, tetris_logic::tetris_logic_shutdown)
            );
    }
}

/*fn test_system(
    mut board: ResMut<TetrisBoard>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let colors = [Colors::Red, Colors::Green, Colors::Blue];

    board.as_mut().set_cell_colour(
        random::<usize>() % BOARD_WIDTH,
        random::<usize>() % BOARD_HEIGHT,
        colors[random::<usize>() % colors.len()],
        materials.as_mut()
    );
}*/

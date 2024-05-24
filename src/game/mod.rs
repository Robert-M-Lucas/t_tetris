use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::sprite::Material2d;
use bevy::window::WindowMode;
use rand::random;
use tetris_board::{Colors, TetrisBoard};
use crate::game::tetris_logic::{ticker_pause, ticker_resume};
use crate::game::ui::update_labels;

use crate::game::ui_setup::{ui_resize_handler, ui_setup};
use crate::GameState;
use crate::util::despawn_screen;

mod ui_setup;
mod tetris_board;
mod tetris_logic;
mod shapes;
mod ui;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 24;
pub const BOARD_WIDTH_F: f32 = BOARD_WIDTH as f32;
pub const BOARD_HEIGHT_F: f32 = BOARD_HEIGHT as f32;

pub const RATIO: f32 = BOARD_WIDTH_F / BOARD_HEIGHT_F;

#[derive(Component)]
pub struct OnGameScreen;

pub struct GamePlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InGameState {
    #[default]
    UnPaused,
    Paused
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameOver {
    #[default]
    NotOver,
    GameOver
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub struct Difficulty {
    pub difficulty: usize
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub struct Score {
    pub score: usize
}


impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<InGameState>()
            .init_state::<GameOver>()
            .init_state::<Difficulty>()
            .init_state::<Score>()
            .add_systems(
                OnEnter(GameState::Game),
                (ui_setup, tetris_board::tetris_board_setup, tetris_logic::tetris_logic_setup)
            )
            .add_systems(
                Update,
                (ui_resize_handler, tetris_board::cell_resize_handler, pause, update_labels)
                    .run_if(in_state(GameState::Game))
            )
            .add_systems(
                Update,
                (tetris_logic::tetris_logic_update)
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(InGameState::UnPaused))
            )
            .add_systems(
                OnEnter(InGameState::UnPaused),
                (ticker_resume)
                    .run_if(in_state(GameState::Game))
            )
            .add_systems(
                OnEnter(InGameState::Paused),
                (ticker_pause)
                    .run_if(in_state(GameState::Game))
            )
            .add_systems(
                OnExit(GameState::Game),
                (despawn_screen::<OnGameScreen>, tetris_board::tetris_board_shutdown, tetris_logic::tetris_logic_shutdown)
            );
    }
}

fn pause(keyboard_input: Res<ButtonInput<KeyCode>>, game_over: Res<State<GameOver>>, state: Res<State<InGameState>>, mut next_state: ResMut<NextState<InGameState>>) {
    if matches!(game_over.as_ref().get(), GameOver::GameOver) { return; }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            InGameState::UnPaused => next_state.set(InGameState::Paused),
            InGameState::Paused => next_state.set(InGameState::UnPaused),
        }
    }
}

fn game_setup(
    mut in_game_state: ResMut<NextState<InGameState>>,
    mut game_over_state: ResMut<NextState<GameOver>>,
    mut difficulty_state: ResMut<NextState<Difficulty>>,
    mut score_state: ResMut<NextState<Score>>,
) {
    in_game_state.set(InGameState::default());
    game_over_state.set(GameOver::default());
    difficulty_state.set(Difficulty::default());
    score_state.set(Score::default());
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

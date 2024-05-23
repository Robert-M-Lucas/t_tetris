use std::array;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use bevy::transform::commands;
use bevy::window::WindowResized;
use rand::random;

use crate::game::ui_setup::{Board, get_target_and_sidebar_width, SideBar, ui_resize_handler, ui_setup};
use crate::GameState;
use crate::util::despawn_screen;

mod ui_setup;

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
                (ui_setup, game_setup)
            )
            .add_systems(
                Update,
                (ui_resize_handler, cell_resize_handler, test_system)
                    .run_if(in_state(GameState::Game))
            )
            .add_systems(
                OnExit(GameState::Game),
                (despawn_screen::<OnGameScreen>, game_shutdown)
            );
    }
}

#[derive(Component)]
pub struct TetrisCell {
    x: usize,
    y: usize
}

impl TetrisCell {
    pub fn new(x: usize, y: usize) -> TetrisCell {
        TetrisCell { x, y }
    }

    pub fn location(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

fn cell_resize_handler(
    mut resize_events: EventReader<WindowResized>,
    mut cells: Query<(&mut Transform, &TetrisCell)>
) {
    let Some(e) = resize_events.read().last() else { return; };
    let cell_width = get_cell_width(e.width, e.height);

    for (mut transform, cell) in cells.iter_mut() {
        let (x, y) = cell.location();
        let pos = get_cell_pos(cell_width, x, y);

        transform.translation = pos.extend(0.0);
        transform.scale = Vec2::splat(cell_width).extend(0.0);
    }
}

fn get_cell_width(width: f32, height: f32) -> f32 {
    let (target_width, _) = get_target_and_sidebar_width(width, height);
    target_width / BOARD_WIDTH_F
}

fn get_cell_pos(cell_width: f32, x: usize, y: usize) -> Vec2 {
    Vec2::new(
        (x as f32 - (BOARD_WIDTH_F / 2.0)) * cell_width + (cell_width / 2.0),
        (y as f32 - (BOARD_HEIGHT_F / 2.0)) * cell_width + (cell_width / 2.0)
    )
}

#[derive(Resource)]
struct TetrisBoard {
    board: [[Handle<ColorMaterial>; BOARD_HEIGHT]; BOARD_WIDTH]
}

impl TetrisBoard {
    pub fn create(
        width: f32,
        height: f32,
        mut commands: &mut Commands,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut materials: &mut ResMut<Assets<ColorMaterial>>
    ) -> TetrisBoard {
        let cell_width = get_cell_width(width, height);
        let colours = [Color::RED, Color::GREEN, Color::BLUE];

        let board = array::from_fn(|x| {
            array::from_fn(|y| {
                let i = y * BOARD_WIDTH + x;

                let pos = get_cell_pos(cell_width, x, y);

                let handle = materials.add(colours[i % colours.len()]);

                commands.spawn((MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::default()).into(),
                    material: handle.clone(),
                    transform: Transform::from_translation(pos.extend(0.0))
                        .with_scale(Vec2::splat(cell_width).extend(1.)),
                    ..default()
                }, OnGameScreen, TetrisCell::new(x, y)));

                handle
            })
        });

        TetrisBoard { board }
    }

    pub fn set_cell_colour(&self, x: usize, y: usize, color: Color, materials: &mut Assets<ColorMaterial>) {
        materials.get_mut(&self.board[x][y]).unwrap().color = color;
    }
}

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {

    let window = window.single();

    let board = TetrisBoard::create(window.width(), window.height(), &mut commands, &mut meshes, &mut materials);
    commands.insert_resource(board);
}

fn game_shutdown(mut commands: Commands) {
    commands.remove_resource::<TetrisBoard>();
}

fn test_system(
    mut board: Res<TetrisBoard>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let colors = [Color::RED, Color::GREEN, Color::BLUE];

    board.set_cell_colour(
        random::<usize>() % BOARD_WIDTH,
        random::<usize>() % BOARD_HEIGHT,
        colors[random::<usize>() % colors.len()],
        materials.as_mut()
    );
}
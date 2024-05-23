use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::game::ui_setup::{get_target_and_sidebar_width, resize_handler, ui_setup};
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
            .add_systems(OnEnter(GameState::Game), (ui_setup, game_setup))
            .add_systems(Update, (resize_handler).run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();
    let (target_width, _) = get_target_and_sidebar_width(window.height(), window.width());
    let cell_width = target_width / BOARD_WIDTH_F;

    let colours = [Color::RED, Color::GREEN, Color::BLUE];

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let i = y * BOARD_WIDTH + x;

            let pos = Vec2::new(
                (x as f32 - (BOARD_WIDTH_F / 2.0)) * cell_width + (cell_width / 2.0),
                (y as f32 - (BOARD_HEIGHT_F / 2.0)) * cell_width + (cell_width / 2.0)
            );

            commands.spawn((MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                material: materials.add(colours[i % colours.len()]),
                transform: Transform::from_translation(pos.extend(0.0))
                    .with_scale(Vec2::splat(cell_width).extend(1.)),
                ..default()
            }, OnGameScreen));
        }
    }

}
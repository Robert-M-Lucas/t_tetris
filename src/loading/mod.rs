use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::util::despawn_screen;
use crate::GameState;

#[derive(Component)]
struct OnLoadingScreen;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalFont::default())
            .add_systems(OnEnter(GameState::Loading), loading_setup)
            .add_systems(
                OnExit(GameState::Loading),
                despawn_screen::<OnLoadingScreen>,
            );
    }
}

#[derive(Default, Resource)]
pub struct GlobalFont {
    font: Option<Handle<Font>>,
}

impl GlobalFont {
    fn set(&mut self, font: Handle<Font>) {
        self.font = Some(font);
    }

    pub fn get(&self) -> Handle<Font> {
        self.font.as_ref().unwrap().clone()
    }

    pub fn get_ref(&self) -> &Handle<Font> {
        self.font.as_ref().unwrap()
    }
}

fn loading_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut font: ResMut<GlobalFont>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    commands.spawn(Camera2d::default());
    font.set(asset_server.load("fonts/FiraSans-Bold.ttf"));
    game_state.set(GameState::Menu);
}

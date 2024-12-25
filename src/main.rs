mod custom_functions;
mod game;
mod loading;
mod menu;
mod util;

// use crate::custom_functions::dev_console_environment;
use crate::game::GamePlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use bevy::ecs::schedule::ExecutorKind;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
// use bevy_dev_console::prelude::ConsoleLogPlugin;
// use bevy_dev_console::DevConsolePlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Game,
    ReloadGame,
}

fn main() {
    let default_plugins = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris".to_string(),
                present_mode: PresentMode::Immediate,
                resolution: default(),
                ..default()
            }),
            ..default()
        })
        .build();

    let mut a = App::new();

    // #[cfg(debug_assertions)]
    // {
    //     let e = dev_console_environment(&mut a);
    //     a.insert_non_send_resource(e);
    // }

    a.add_plugins((
        // Start capturing logs before the default plugins initiate.
        // #[cfg(debug_assertions)]
        // ConsoleLogPlugin::default(),
        // Add the default plugins without the LogPlugin.
        // Not removing the LogPlugin will cause a panic!
        #[cfg(debug_assertions)]
        default_plugins.disable::<LogPlugin>(),
        #[cfg(not(debug_assertions))]
        default_plugins,
        // Add the dev console plugin itself.
        // #[cfg(debug_assertions)]
        // DevConsolePlugin,
    ))
    .insert_resource(ClearColor(Color::WHITE))
    .init_state::<GameState>()
    .add_systems(Update, fullscreen)
    .add_plugins((LoadingPlugin, MenuPlugin, GamePlugin))
    .run();
}

fn fullscreen(keyboard_input: Res<ButtonInput<KeyCode>>, mut window: Query<&mut Window>) {
    if keyboard_input.just_pressed(KeyCode::F11) {
        let mut window = window.single_mut();
        match &window.mode {
            WindowMode::Fullscreen(_) => {
                window.mode = WindowMode::Windowed;
                window.resolution = default();
            }
            _ => window.mode = WindowMode::Fullscreen(MonitorSelection::Current),
        };
    }
}

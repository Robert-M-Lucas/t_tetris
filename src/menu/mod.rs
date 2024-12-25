use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::loading::GlobalFont;
use crate::util::despawn_screen;
use crate::GameState;

#[derive(Component)]
struct OnMenuScreen;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(Update, (play_button).run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>);
    }
}

#[derive(Component)]
struct PlayButton;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

fn play_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>, With<PlayButton>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                game_state.set(GameState::Game);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn menu_setup(mut commands: Commands, font: Res<GlobalFont>) {
    // Root node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            // Tetris text
            // parent.spawn((TextBundle::from_section(
            //     "Tetris",
            //     TextStyle {
            //         font: font.get(),
            //         font_size: 40.0,
            //         color: Color::BLACK,
            //     }
            // )));

            parent.spawn((
                Text("Tetris".to_string()),
                TextFont {
                    font: font.get(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            // Play button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BackgroundColor(NORMAL_BUTTON.into()),
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text("Play".to_string()),
                        TextFont {
                            font: font.get(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        });
}

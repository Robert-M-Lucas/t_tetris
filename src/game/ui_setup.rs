use crate::game::{OnGameScreen, RATIO};
use crate::loading::GlobalFont;
use bevy::prelude::*;
use bevy::window::WindowResized;

#[derive(Component)]
pub struct Board;

#[derive(Component)]
pub struct SideBar;

#[derive(Component)]
pub struct ScoreLabel;

#[derive(Component)]
pub struct DifficultyLabel;

#[derive(Component)]
pub struct InfoLabel;

pub fn get_target_and_sidebar_width(width: f32, height: f32) -> (f32, f32) {
    let target_width = height * RATIO;
    let sidebar_width = (width - target_width) / 2.0;
    (target_width, sidebar_width)
}

pub fn ui_resize_handler(
    mut resize_events: EventReader<WindowResized>,
    mut sidebars: Query<&mut Node, (With<SideBar>, Without<Board>)>,
    mut board: Query<&mut Node, (With<Board>, Without<SideBar>)>,
) {
    let Some(e) = resize_events.read().last() else {
        return;
    };

    let (target_width, sidebar_width) = get_target_and_sidebar_width(e.width, e.height);

    for mut s in sidebars.iter_mut() {
        s.width = Val::Px(sidebar_width);
    }

    board.single_mut().width = Val::Px(target_width);
}

pub fn ui_setup(mut commands: Commands, font: Res<GlobalFont>, window: Query<&Window>) {
    let window = window.single();

    let (target_width, sidebar_width) =
        get_target_and_sidebar_width(window.width(), window.height());

    // root node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Stretch,
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn((
                    Node {
                        height: Val::Percent(100.),
                        width: Val::Px(sidebar_width),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.65, 0.65, 0.65).into()),
                    SideBar,
                ))
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(Node {
                            width: Val::Percent(100.),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                Text("Score: 0".to_string()),
                                TextFont {
                                    font: font.get(),
                                    font_size: 30.0,
                                    ..default()
                                },
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                                ScoreLabel,
                            ));

                            parent.spawn((
                                Text("Difficulty: 1".to_string()),
                                TextFont {
                                    font: font.get(),
                                    font_size: 30.0,
                                    ..default()
                                },
                                Node {
                                    margin: UiRect::all(Val::Px(5.)),
                                    ..default()
                                },
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                                DifficultyLabel,
                            ));
                        });
                });

            // centre
            parent
                .spawn((
                    Node {
                        height: Val::Percent(100.),
                        width: Val::Px(target_width),
                        ..default()
                    },
                    Board,
                ))
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(Node {
                            width: Val::Percent(100.),
                            ..default()
                        })
                        .with_children(|_parent| {
                            // text
                            // parent.spawn((
                            //     TextBundle::from_section(
                            //         "Centre",
                            //         TextStyle {
                            //             font: font.get(),
                            //             font_size: 30.0,
                            //             ..default()
                            //         },
                            //     ),
                            //     // Because this is a distinct label widget and
                            //     // not button/list item text, this is necessary
                            //     // for accessibility to treat the text accordingly.
                            //     Label,
                            // ));
                        });
                });

            // right vertical fill (border)
            parent
                .spawn((
                    Node {
                        height: Val::Percent(100.),
                        width: Val::Px(sidebar_width),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.65, 0.65, 0.65).into()),
                    SideBar,
                ))
                .with_children(|parent| {
                    // right vertical fill (content)
                    parent
                        .spawn(Node {
                            width: Val::Percent(100.),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                Text("Playing".to_string()),
                                TextFont {
                                    font: font.get(),
                                    font_size: 30.0,
                                    ..default()
                                },
                                Node {
                                    margin: UiRect::all(Val::Px(20.)),
                                    ..default()
                                },
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                                InfoLabel,
                            ));

                            parent.spawn((
                                Text(include_str!("instructions.txt").to_string()),
                                TextFont {
                                    font: font.get(),
                                    font_size: 30.0,
                                    ..default()
                                },
                                Node {
                                    margin: UiRect::all(Val::Px(5.)),
                                    ..default()
                                },
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));
                        });
                });
        });
}

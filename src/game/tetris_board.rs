use crate::game::ui_setup::get_target_and_sidebar_width;
use crate::game::{OnGameScreen, BOARD_HEIGHT, BOARD_HEIGHT_F, BOARD_WIDTH, BOARD_WIDTH_F};
use bevy::asset::{Assets, Handle};
use bevy::color::palettes::css;
use bevy::math::Vec2;
use bevy::prelude::{
    default, Color, ColorMaterial, Commands, Component, EventReader, Mesh, MeshMaterial2d, Query,
    Rectangle, ResMut, Resource, Transform, Window,
};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResized;
use std::array;

#[derive(Component)]
pub struct TetrisCell {
    x: usize,
    y: usize,
}

impl TetrisCell {
    pub fn new(x: usize, y: usize) -> TetrisCell {
        TetrisCell { x, y }
    }

    pub fn location(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub fn cell_resize_handler(
    mut resize_events: EventReader<WindowResized>,
    mut cells: Query<(&mut Transform, &TetrisCell)>,
) {
    let Some(e) = resize_events.read().last() else {
        return;
    };
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
        (y as f32 - (BOARD_HEIGHT_F / 2.0)) * cell_width + (cell_width / 2.0),
    )
}

#[derive(Copy, Clone)]
pub enum Colors {
    Empty,
    LightBlue,
    Blue,
    Orange,
    Yellow,
    Lime,
    Purple,
    Red,
}

// impl Colors {
//     pub fn random_not_empty() -> Colors {
//         match thread_rng().gen_range(0..3) {
//             0 => Colors::Red,
//             1 => Colors::Lime,
//             2 => Colors::Blue,
//             _ => unreachable!()
//         }
//     }
// }

impl Colors {
    fn get_color(&self) -> Color {
        match &self {
            Colors::Empty => Color::BLACK,
            Colors::Red => css::RED.into(),
            Colors::Lime => css::LIME.into(),
            Colors::Blue => css::BLUE.into(),
            Colors::LightBlue => css::TEAL.into(),
            Colors::Orange => css::ORANGE.into(),
            Colors::Yellow => css::YELLOW.into(),
            Colors::Purple => css::PURPLE.into(),
        }
    }
}

#[derive(Resource)]
pub struct TetrisBoard {
    board_materials: [[Handle<ColorMaterial>; BOARD_HEIGHT]; BOARD_WIDTH],
    board: [[Colors; BOARD_HEIGHT]; BOARD_WIDTH],
}

impl TetrisBoard {
    pub fn create(
        width: f32,
        height: f32,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> TetrisBoard {
        let cell_width = get_cell_width(width, height);

        let board_materials = array::from_fn(|x| {
            array::from_fn(|y| {
                // let i = y * BOARD_WIDTH + x;

                let pos = get_cell_pos(cell_width, x, y);

                let handle = materials.add(Color::BLACK);

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Rectangle::default()).into(),
                        material: MeshMaterial2d(handle.clone()),
                        transform: Transform::from_translation(pos.extend(0.0))
                            .with_scale(Vec2::splat(cell_width).extend(1.)),
                        ..default()
                    },
                    OnGameScreen,
                    TetrisCell::new(x, y),
                ));

                handle
            })
        });

        let board = [[Colors::Empty; BOARD_HEIGHT]; BOARD_WIDTH];

        TetrisBoard {
            board_materials,
            board,
        }
    }

    pub fn set_cell_colour(
        &mut self,
        x: usize,
        y: usize,
        color: Colors,
        materials: &mut Assets<ColorMaterial>,
    ) {
        materials
            .get_mut(&self.board_materials[x][y])
            .unwrap()
            .color = color.get_color();
        self.board[x][y] = color;
    }

    pub fn board(&self) -> &[[Colors; BOARD_HEIGHT]; BOARD_WIDTH] {
        &self.board
    }
}

pub fn tetris_board_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();
    let board = TetrisBoard::create(
        window.width(),
        window.height(),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    commands.insert_resource(board);
}

pub fn tetris_board_shutdown(mut commands: Commands) {
    commands.remove_resource::<TetrisBoard>();
}

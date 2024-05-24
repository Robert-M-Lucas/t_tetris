use std::cmp::{max, min};
use bevy::asset::Assets;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy_dev_console::ui::ConsoleUiState;
use rand::random;
use rand_derive2::RandGen;
use log::log;
use crate::game::{BOARD_HEIGHT, BOARD_WIDTH, Difficulty, GameOver, InGameState, Score};
use crate::game::shapes::{BACK_L_SHAPE, BACK_Z_SHAPE, L_SHAPE, LINE, SQUARE, T_SHAPE, Z_SHAPE};
use crate::game::tetris_board::{Colors, TetrisBoard};

#[derive(Debug, RandGen)]
enum Tetrominos {
    LShape,
    BackLShape,
    Line,
    Square,
    ZShape,
    BackZShape,
    TShape
}

impl Tetrominos {
    pub fn get_shape(&self) -> &'static [[[bool; 4]; 4]; 4] {
        match &self {
            Tetrominos::LShape => &L_SHAPE,
            Tetrominos::BackLShape => &BACK_L_SHAPE,
            Tetrominos::Line => &LINE,
            Tetrominos::Square => &SQUARE,
            Tetrominos::ZShape => &Z_SHAPE,
            Tetrominos::BackZShape => &BACK_Z_SHAPE,
            Tetrominos::TShape => &T_SHAPE,
        }
    }

    pub fn get_color(&self) -> Colors {
        match &self {
            Tetrominos::LShape => Colors::Orange,
            Tetrominos::BackLShape => Colors::Blue,
            Tetrominos::Line => Colors::LightBlue,
            Tetrominos::Square => Colors::Yellow,
            Tetrominos::ZShape => Colors::Red,
            Tetrominos::BackZShape => Colors::Lime,
            Tetrominos::TShape => Colors::Purple,
        }
    }
}

#[derive(Resource)]
pub struct Ticker {
    last: f32,
    save_point: Option<f32>,
    interval: f32,
}

impl Ticker {
    pub fn new(time: &Time, interval: f32) -> Ticker {
        Ticker {
            last: time.elapsed_seconds(),
            save_point: None,
            interval
        }
    }

    pub fn ticks(&mut self, time: &Time) -> usize {
        let ticks = ((time.elapsed_seconds() - self.last) / self.interval) as usize;

        self.last += self.interval * ticks as f32;

        ticks
    }

    pub fn set_interval(&mut self, interval: f32) {
        self.interval = interval;
    }

    pub fn pause(&mut self, time: &Time) {
        self.save_point = Some(time.elapsed_seconds());
    }

    pub fn resume(&mut self, time: &Time) {
        if let Some(save_point) = self.save_point {
            self.last = time.elapsed_seconds() - (save_point - self.last);
        }
    }
}

pub fn ticker_pause(mut ticker: ResMut<Ticker>, time: Res<Time>) {
    ticker.pause(&time);
}

pub fn ticker_resume(mut ticker: ResMut<Ticker>, time: Res<Time>) {
    ticker.resume(&time);
}

#[derive(Resource)]
pub struct TetrisLogic {
    x: i32,
    y: i32,
    rot: usize,
    current_shape: Option<Tetrominos>,
    current_color: Colors,
    difficulty: usize,
    score: usize
}


impl TetrisLogic {
    pub fn new() -> TetrisLogic {
        TetrisLogic {
            x: 0,
            y: 0,
            rot: 0,
            current_shape: None,
            current_color: Colors::Red,
            difficulty: 0,
            score: 0
        }
    }

    pub fn update(&mut self, board: &mut TetrisBoard, keyboard: &ButtonInput<KeyCode>, materials: &mut Assets<ColorMaterial>) {
        if self.current_shape.is_some() {
            if keyboard.just_pressed(KeyCode::ArrowDown) {
                self.down(board, materials);
            }
            if keyboard.just_pressed(KeyCode::ArrowUp) {
                self.snap_down(board, materials);
            }
            if keyboard.just_pressed(KeyCode::ArrowLeft) {
                self.left(board, materials);
            }
            if keyboard.just_pressed(KeyCode::ArrowRight) {
                self.right(board, materials);
            }
            if keyboard.just_pressed(KeyCode::KeyE) {
                self.clockwise(board, materials);
            }
            if keyboard.just_pressed(KeyCode::KeyQ) {
                self.anticlockwise(board, materials);
            }
        }
    }

    pub fn tick(
        &mut self,
        board: &mut TetrisBoard,
        materials: &mut Assets<ColorMaterial>,
        difficulty: &mut NextState<Difficulty>,
        score: &mut NextState<Score>,
        ticker: &mut Ticker,
        in_game_state: &mut NextState<InGameState>,
        game_over_state: &mut NextState<GameOver>
    ) {
        if self.current_shape.is_none() {
            if !self.spawn(board, materials) {
                in_game_state.set(InGameState::Paused);
                game_over_state.set(GameOver::GameOver);
            }
        }
        else if !self.down(board, materials) {
            let clears = self.check_clear(board, materials);
            let score_a = if clears == 0 {
                0
            } else {
                100 * 2usize.pow(clears - 1)
            };

            self.score += score_a;
            score.set(Score { score: self.score });

            self.difficulty += clears as usize;
            difficulty.set(Difficulty { difficulty: self.difficulty });

            ticker.set_interval(self.get_interval());

            self.current_shape = None;

            if !self.spawn(board, materials) {
                in_game_state.set(InGameState::Paused);
                game_over_state.set(GameOver::GameOver);
            }
        }
    }

    //noinspection ALL
    fn check_clear(&mut self, board: &mut TetrisBoard, materials: &mut Assets<ColorMaterial>) -> u32 {
        let c_board = board.board();
        let mut found = None;

        'outer: for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if matches!(c_board[x][y], Colors::Empty) {
                    continue 'outer;
                }
            }

            found = Some(y);
            break;
        }

        if let Some(found) = found {
            if found != BOARD_HEIGHT - 1 {
                for y in found+1..BOARD_HEIGHT {
                    for x in 0..BOARD_WIDTH {
                        let color = board.board()[x][y];
                        board.set_cell_colour(x, y-1, color, materials);
                    }
                }

                1 + self.check_clear(board, materials)
            }
            else {
                1
            }
        }
        else {
            0
        }
    }

    fn get_interval(&self) -> f32 {
        1.0 / ((self.difficulty as f32 / 10.0) + 2.0)
    }
    fn spawn(&mut self, board: &mut TetrisBoard, materials: &mut Assets<ColorMaterial>) -> bool {
        if self.current_shape.is_some() {
            panic!("Tried to spawn tetromino while one is in play!");
        }

        self.current_shape = Some(random());
        self.current_color = self.current_shape.as_ref().unwrap().get_color();
        self.x = ((BOARD_WIDTH / 2) - (4 / 2) )as i32;
        self.y = (BOARD_HEIGHT - 1) as i32;
        self.rot = 0;

        if !self.test(board) {
            return false;
        }

        self.draw(board, materials);

        true
    }

    fn draw(&mut self, board: &mut TetrisBoard, materials: &mut Assets<ColorMaterial>) {
        let shape = &self.current_shape.as_ref().unwrap().get_shape()[self.rot];

        for x in 0..4 {
            for y in 0..4 {
                if shape[y][x] {
                    let true_x = self.x + x as i32;
                    let true_y = self.y - y as i32;

                    if true_x >= 0 && true_y >= 0 && true_x < BOARD_WIDTH as i32 && true_y < BOARD_HEIGHT as i32 {
                        board.set_cell_colour(true_x as usize, true_y as usize, self.current_color, materials);
                    }
                }
            }
        }
    }

    fn undraw(&mut self, board: &mut TetrisBoard, materials: &mut Assets<ColorMaterial>) {
        let shape = &self.current_shape.as_ref().unwrap().get_shape()[self.rot];

        for x in 0..4 {
            for y in 0..4 {
                if shape[y][x] {
                    let true_x = self.x + x as i32;
                    let true_y = self.y - y as i32;

                    if true_x >= 0 && true_y >= 0 && true_x < BOARD_WIDTH as i32 && true_y < BOARD_HEIGHT as i32 {
                        board.set_cell_colour(true_x as usize, true_y as usize, Colors::Empty, materials);
                    }
                }
            }
        }
    }

    fn test(&self, board: &TetrisBoard) -> bool {
        let shape = &self.current_shape.as_ref().unwrap().get_shape()[self.rot];

        for x in 0..4 {
            for y in 0..4 {
                if shape[y][x] {
                    let true_x = self.x + x as i32;
                    let true_y = self.y - y as i32;

                    if true_x < 0 ||
                        true_y < 0 ||
                        true_x >= BOARD_WIDTH as i32 ||
                        true_y >= BOARD_HEIGHT as i32 ||
                        !matches!(&board.board()[true_x as usize][true_y as usize], Colors::Empty) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn down(&mut self, board: &mut TetrisBoard, materials: &mut Assets<ColorMaterial>) -> bool {
        self.undraw(board, materials);
        self.y -= 1;

        let res = self.test(board);
        if !res {
            self.y += 1;
        }

        self.draw(board, materials);
        res
    }

    fn snap_down(&mut self, board: &mut TetrisBoard, materials: &mut Assets<ColorMaterial>) {
        while self.down(board, materials) {}
    }

    fn left(&mut self, board: &mut TetrisBoard, materials: &mut Assets<ColorMaterial>) -> bool {
        self.undraw(board, materials);
        self.x -= 1;

        let res = self.test(board);
        if !res {
            self.x += 1;
        }

        self.draw(board, materials);
        res
    }

    fn right(&mut self, board: &mut TetrisBoard, materials: &mut Assets<ColorMaterial>) -> bool {
        self.undraw(board, materials);
        self.x += 1;

        let res = self.test(board);
        if !res {
            self.x -= 1;
        }

        self.draw(board, materials);
        res
    }

    fn clockwise(&mut self, board: &mut TetrisBoard, materials: &mut Assets<ColorMaterial>) -> bool {
        self.undraw(board, materials);
        self.rot = (self.rot + 1) % 4;

        let res = self.test(board);
        if !res {
            self.x += 1;
            if !self.test(board) {
                self.x -= 2;
                if !self.test(board) {
                    self.x += 1;
                    self.rot = match self.rot {
                        0 => 3,
                        r => r - 1,
                    };
                }
            }
        }

        self.draw(board, materials);
        res
    }

    fn anticlockwise(&mut self, board: &mut TetrisBoard, materials: &mut Assets<ColorMaterial>) -> bool {
        self.undraw(board, materials);
        self.rot = match self.rot {
            0 => 3,
            r => r - 1,
        };

        let res = self.test(board);
        if !res {
            if !res {
                self.x += 1;
                if !self.test(board) {
                    self.x -= 2;
                    if !self.test(board) {
                        self.x += 1;
                        self.rot = (self.rot + 1) % 4;
                    }
                }
            }
        }

        self.draw(board, materials);
        res
    }
}

pub fn tetris_logic_setup(mut commands: Commands, time: Res<Time>) {
    let logic = TetrisLogic::new();
    commands.insert_resource(Ticker::new(&time, logic.get_interval()));
    commands.insert_resource(logic);
}

pub fn tetris_logic_update(
    mut difficulty: ResMut<NextState<Difficulty>>,
    mut score: ResMut<NextState<Score>>,
    mut logic: ResMut<TetrisLogic>,
    mut board: ResMut<TetrisBoard>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ticker: ResMut<Ticker>,
    mut game_over_state: ResMut<NextState<GameOver>>,
    mut in_game_state: ResMut<NextState<InGameState>>,
    time: Res<Time>,
    #[cfg(debug_assertions)]
    console: Res<ConsoleUiState>
) {
    for _ in 0..ticker.as_mut().ticks(&time) {
        logic.as_mut().tick(&mut board, &mut materials, &mut difficulty, &mut score, &mut ticker, &mut in_game_state, &mut game_over_state);
    }

    #[cfg(debug_assertions)]
    if !console.open() {
        logic.as_mut().update(board.as_mut(), &keyboard, materials.as_mut());
    }

    #[cfg(not(debug_assertions))]
    logic.as_mut().update(board.as_mut(), &keyboard, materials.as_mut());
}

pub fn tetris_logic_shutdown(mut commands: Commands) {
    commands.remove_resource::<TetrisLogic>();
    commands.remove_resource::<Ticker>();
}
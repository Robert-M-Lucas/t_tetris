use bevy::prelude::*;
use crate::game::{Difficulty, GameOver, InGameState, Score};
use crate::game::ui_setup::{DifficultyLabel, InfoLabel, ScoreLabel};

pub fn update_labels(
    mut score_label: Query<&mut Text, (With<ScoreLabel>, Without<DifficultyLabel>, Without<InfoLabel>)>,
    mut difficulty_label: Query<&mut Text, (With<DifficultyLabel>, Without<ScoreLabel>, Without<InfoLabel>)>,
    mut info_label: Query<&mut Text, (With<InfoLabel>, Without<DifficultyLabel>, Without<ScoreLabel>)>,
    score: Res<State<Score>>,
    difficulty: Res<State<Difficulty>>,
    game_over: Res<State<GameOver>>,
    in_game: Res<State<InGameState>>
) {
    score_label.single_mut().sections[0].value = format!("Score: {}", score.score);
    difficulty_label.single_mut().sections[0].value = format!("Difficulty: {}", difficulty.difficulty + 1);

    let playing_text = if matches!(game_over.get(), GameOver::GameOver) {
        "Game Over"
    } else {
        "Playing"
    };
    let state_text = if matches!(in_game.get(), InGameState::Paused) {
        "\n[Paused, Restart : R, Main Menu : Return]"
    } else {
        ""
    };

    info_label.single_mut().sections[0].value = format!("{playing_text}{state_text}");
}
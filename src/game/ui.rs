use bevy::prelude::*;
use crate::game::{Difficulty, Score};
use crate::game::ui_setup::{DifficultyLabel, ScoreLabel};

pub fn update_labels(
    mut score_label: Query<&mut Text, (With<ScoreLabel>, Without<DifficultyLabel>)>,
    mut difficulty_label: Query<&mut Text, (With<DifficultyLabel>, Without<ScoreLabel>)>,
    score: Res<State<Score>>,
    difficulty: Res<State<Difficulty>>
) {
    score_label.single_mut().sections[0].value = format!("Score: {}", score.score);
    difficulty_label.single_mut().sections[0].value = format!("Difficulty: {}", difficulty.difficulty + 1);
}
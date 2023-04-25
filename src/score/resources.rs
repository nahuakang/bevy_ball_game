use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub struct Score {
    pub value: u32,
}

#[derive(Debug, Default, Resource)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

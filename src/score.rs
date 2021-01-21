use crate::consts::*;

#[derive(Default)]
pub struct ScoreResource {
    corrects: usize,
    fails: usize,

    score: usize,
}

impl ScoreResource {
    /// Increases number of corrects and adds to score
    pub fn increase_correct(&mut self, distance: f32) -> usize {
        self.corrects += 1;

        // Get a value from 0 to 1 according to how close the press was
        let score_multiplier = (THRESHOLD - distance.abs()) / THRESHOLD;
        // Give at least 10 points and 100 at max
        let points = (score_multiplier * 100.).min(100.).max(10.) as usize;
        self.score += points;

        points
    }

    /// Increases number of failures
    pub fn increase_fails(&mut self) {
        self.fails += 1;
    }

    // Getters

    pub fn score(&self) -> usize {
        self.score
    }
    pub fn corrects(&self) -> usize {
        self.corrects
    }
    pub fn fails(&self) -> usize {
        self.fails
    }
}

pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        let l = self.scores.len();
        match l {
            0 => None,
            _ => Some(self.scores[self.scores.len() - 1]),
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().cloned().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scrs = self.scores.clone();
        scrs.sort_by(|a, b| b.cmp(a));
        scrs.truncate(3);
        scrs
    }
}
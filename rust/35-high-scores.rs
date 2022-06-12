#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }
    pub fn scores(&self) -> &[u32] {
        self.scores
    }
    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }
    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }
    pub fn personal_top_three(&self) -> Vec<u32> {
        self.scores.iter().fold([0, 0, 0], |[a, b, c], &x| {
            if x > a { [x, a, b] }
            else if x > b { [a, x, b] }
            else if x > c { [a, b, x] }
            else { [a, b, c] }
        })[..self.scores.len().min(3)]
            .to_vec()
    }
}

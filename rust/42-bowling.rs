#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    throws: Vec<u16>,
    second: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            throws: Vec::with_capacity(21),
            second: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        use Error::*;
        if (self.second && pins + self.throws.last().unwrap() > 10) || pins > 10 {
            Err(NotEnoughPinsLeft)
        } else if self.score().is_some() {
            Err(GameComplete)
        } else {
            self.throws.push(pins);
            self.second = !self.second && pins != 10;
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut cr = 0;
        let mut sum = 0;
        for _ in 1..=10 {
            sum += *self.throws.get(cr)? + *self.throws.get(cr + 1)?;
            if self.throws[cr] + self.throws[cr + 1] >= 10 {
                sum += *self.throws.get(cr + 2)?;
            }
            cr += if self.throws[cr] == 10 { 1 } else { 2 };
        }
        Some(sum)
    }
}

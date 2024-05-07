use rand::Rng;

pub struct Dice {
    sides: u32,
}

impl Dice {
    pub fn new(sides: u32) -> Dice {
        Dice { sides }
    }

    pub fn roll(&self) -> u32 {
        return rand::thread_rng().gen_range(0..=self.sides)
    }
}
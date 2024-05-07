use crate::core::dice_cup::dice::application::dice_response::DiceResponse;
use crate::core::dice_cup::dice::domain::dice::Dice;

pub struct DiceRoller {
    sides: String,
    times: String,
}

impl DiceRoller {
    pub fn new(sides: String, times: Option<String>) -> DiceRoller {
        DiceRoller {
            sides,
            times: times.unwrap_or_else(|| 1.to_string()),
        }
    }

    pub fn process(&self) -> DiceResponse {
        let dice_roll_times: u8 = self.times.parse().unwrap();
        let dice_sides: u32     = self.sides.parse().unwrap();
        let dice                = Dice::new(dice_sides);
        let mut results         = Vec::new();

        for _ in 0..dice_roll_times {
            results.push(dice.roll());
        }

        return DiceResponse { results };
    }

}

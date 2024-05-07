use actix_web::{HttpResponse, Responder};
use actix_web::web::Query;
use serde::Deserialize;

use crate::apps::roll_d_n_rust_api::validator::ValidationException;
use crate::core::dice_cup::dice::application::dice_roller::DiceRoller;

#[derive(Deserialize)]
pub struct RollDiceQuery {
    pub sides: String,
    pub times: String,
}

impl RollDiceQuery {
    fn validate(&self) -> Result<(), ValidationException> {
        crate::apps::roll_d_n_rust_api::validator::validate_not_empty(&self.sides, "sides")?;
        crate::apps::roll_d_n_rust_api::validator::validate_is_numeric(&self.sides, "sides")?;

        crate::apps::roll_d_n_rust_api::validator::validate_not_empty(&self.times, "times")?;
        crate::apps::roll_d_n_rust_api::validator::validate_is_numeric(&self.times, "times")?;

        Ok(())
    }
}

pub async fn roll_dice(query: Query<RollDiceQuery>) -> impl Responder {
    if let Err(e) = query.validate() {
        return HttpResponse::BadRequest().json(e.to_string());
    }

    let dice_roller = DiceRoller::new(
        query.sides.to_string(),
        Some(query.times.to_string()),
    );
    let response = dice_roller.process();

    HttpResponse::Ok().json(response)
}
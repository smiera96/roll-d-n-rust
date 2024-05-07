use serde::Serialize;

#[derive(Serialize)]
pub struct DiceResponse {
    pub results: Vec<u32>,
}
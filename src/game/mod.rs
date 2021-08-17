use crate::model;
use model::interval::Interval;

pub fn get_game_info() -> String {
    let msg_from_model = model::get_model_info();
    let i = Interval::Second;
    format!(
        "interval {:?} message from game containing message from model: {}",
        i, msg_from_model
    )
}

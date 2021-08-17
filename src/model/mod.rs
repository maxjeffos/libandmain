pub mod interval;
use interval::Interval;

pub fn get_model_info() -> String {
    let i = Interval::Third;
    format!("message from model: {:?}", i)
}

use libandmain::{game, model};

fn main() {
    println!("hello");
    let g = game::get_game_info();
    let m = model::get_model_info();

    println!("g: {}", g);
    println!("m: {}", m);
}

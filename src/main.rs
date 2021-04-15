mod learn_lib;
use learn_lib::guess_game;
use learn_lib::prog_3_1;
use learn_lib::prog_3_2;
use learn_lib::prog_3_3;
use learn_lib::prog_3_5;
use std::io;

fn main() {

    println!("Please input program type:");

    let mut input_type = String::new();

    io::stdin().read_line(&mut input_type).expect("Failed to read line.");

    match input_type.trim().as_ref() {
        "guess_number" => guess_game::guess_number(),
        "3_1" => {
            prog_3_1::action();
            prog_3_1::action_shadow();
        }
        "3_2" => {
            prog_3_2::action();
            prog_3_2::action_panic();
        }
        "3_3" => {
            prog_3_3::action();
            prog_3_3::action_block();
            prog_3_3::function_with_return();
        }
        "3_5" => {
            prog_3_5::action();
            prog_3_5::action_loop();
            prog_3_5::action_while();
            prog_3_5::action_for();
            prog_3_5::action_for_range();
            prog_3_5::action_f_arr();
        }
        _ => println!("other program"),
    };
}

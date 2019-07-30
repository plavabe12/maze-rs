// Copyright © 2019 Ebraheem AlAthari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// Rust Declarations
mod screen;

// Structs

// Implementations

// Functions
fn main() {
    let max_terminal: screen::TerminalScreen = screen::get_terminal_size();
    let mut game_output: screen::TerminalScreen = screen::get_terminal_size();
    println!("{} {}", max_terminal.width, max_terminal.height);
    println!("{} {}", game_output.width, game_output.height);

    let new_game: Vec<Vec<String>> = screen::build_game_screen(&mut game_output);
    println!("{} {}", game_output.width, game_output.height);
    screen::build_maze(&new_game, &game_output);
    screen::print_maze(&new_game);
    // loop {
    //     print_maze(&new_game);
    // }
}

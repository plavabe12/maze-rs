// Copyright © 2019 Ebraheem AlAthari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// *** Rust Declarations ***
mod player;
mod screen;
mod welcome;

// *** Functions ***
fn main() {
    welcome::welcome_msg();
    let mut game_output: screen::TerminalScreen = screen::get_terminal_size();
    let default_maze: Vec<Vec<String>> = screen::build_game_screen(&mut game_output);

    loop {
        println!("Generating New Maze");
        let mut new_game: Vec<Vec<String>> = default_maze.clone();
        screen::build_maze(&mut new_game, &game_output);
        player::play_game(&mut new_game, &game_output);
    }
}

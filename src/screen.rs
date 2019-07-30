// Copyright © 2019 Ebraheem AlAthari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// Rust Declarations
use terminal_size::{Width, Height, terminal_size};

// Structs
pub struct TerminalScreen {
    pub width: usize,
    pub height: usize
}

// Public Functions
pub fn get_terminal_size() -> TerminalScreen {
    // Get Terminal Size from the extern crate terminal_size
    // Return correct size if it can determine terminal window
    if let Some((Width(w), Height(h))) = terminal_size() {
        if w <= 9 || h <= 9 {
            panic!("Terminal Screen too small, Please set it to atleast a 10 by 10 screen");
        }
        TerminalScreen { width: w as usize, height: h as usize }
    } else {
        panic!("Terminal Screen not detected");
    }
}

pub fn build_game_screen(mut terminal: &mut TerminalScreen) -> Vec<Vec<String>> {
    //Adjust Terminal Window to fit game
    terminal.width -= 2usize;
    terminal.height -= 2usize;

    //Bulding the vector string
    build_box(&terminal)
}

pub fn build_maze(_maze: &Vec<Vec<String>>, _game_output: &TerminalScreen) {
    unimplemented!()
}

pub fn print_maze(maze: & Vec<Vec<String>>) {
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            print!("{}",maze[i][j]);
        }
        println!();
    }
}

// Private Functions
fn  build_box(terminal: &TerminalScreen) -> Vec<Vec<String>> {
    //Building a basic box for the maze
    //Borrowed Unicode characters from https://github.com/boppreh/maze
    let mut new_box: Vec<Vec<String>> = vec![vec!['X'.to_string(); terminal.width]; terminal.height];
    for i in 0..new_box.len() {
        for j in 0..new_box[i].len() {
            if i == 0 && j == 0 {
                new_box[i][j] = '┌'.to_string();
            } else if i == new_box.len() - 1usize && j == 0  {
                new_box[i][j] = '└'.to_string();
            } else if i == 0 && j == new_box[i].len() - 1usize {
                new_box[i][j] = '┐'.to_string();
            } else if i == new_box.len() - 1usize && j == new_box[i].len() - 1usize {
                new_box[i][j] = '┘'.to_string();
            } else if i == 0usize || i == new_box.len() - 1usize {
                new_box[i][j] = '─'.to_string();
            } else if j == 0usize || j == new_box[i].len() - 1usize {
                new_box[i][j] = '│'.to_string();
            }
        }
    }
    new_box
}

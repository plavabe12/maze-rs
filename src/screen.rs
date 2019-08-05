// Copyright © 2019 Ebraheem AlAthari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// Rust Declarations
use terminal_size::{Width, Height, terminal_size};
use rand::{self, Rng};

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

pub fn build_maze(maze: &mut std::vec::Vec<std::vec::Vec<std::string::String>>, game_output: &TerminalScreen) {
    // Computing Total Nodes using the game output and subtracting the sides
    //let mut total_nodes: u64 = ((game_output.height - 2usize) * (game_output.width - 2usize)) as u64;

    // Generate Starting Point
    // Min: (1,1)
    // Max: (game_output.height - 2usize,game_output.width - 2usize)
    let new_point: u64 = rand::thread_rng().gen_range(1, 4);
    let mut start_point: (usize,usize) = match new_point {
        1 => (game_output.height - 2usize, rand::thread_rng().gen_range(1, game_output.width - 2usize) as usize), // Bottom Start
        2 => (1usize,  rand::thread_rng().gen_range(1, game_output.width - 2usize) as usize), // Top Start
        3 => (rand::thread_rng().gen_range(1, game_output.height - 2usize) as usize, 1usize), // Right Start
        4 => (rand::thread_rng().gen_range(1, game_output.height - 2usize) as usize, game_output.width - 2usize), // Left Start
        _ => (0usize, 0usize) // Failed Start
    };

    //let start_point_copy: (usize,usize) = start_point.clone();

    maze[start_point.0][start_point.1] = " ".to_string();
    visited(maze, &start_point);

    loop {
        match new_point {
            1 => {
                start_point = (start_point.0 - 1usize, start_point.1);
                if start_point.0 == 0usize { break }
            }, // Bottom Start
            2 => {
                start_point = (start_point.0 + 1usize, start_point.1);
                if start_point.0 == game_output.height - 1usize { break }
            }, // Top Start
            3 => {
                start_point = (start_point.0, start_point.1 + 1usize);
                if start_point.1 == game_output.width - 1usize { break }
            }, // Right Start
            4 => {
                start_point = (start_point.0, start_point.1 - 1usize);
                if start_point.1 == 0usize { break }
            }, // Left Start
            _ => () // Failed Start
        };
        maze[start_point.0][start_point.1] = " ".to_string();
        visited(maze, &start_point);
        //total_nodes -= 1;
    }
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
    let mut new_box: Vec<Vec<String>> = vec![vec!['O'.to_string(); terminal.width]; terminal.height];
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

fn visited(maze: &mut std::vec::Vec<std::vec::Vec<std::string::String>>, point: &(usize,usize)) {
    let mut adjusted_point: (usize,usize) = point.clone();
    for direction in 0..4 {
        match direction {
            0 => {
                adjusted_point = (adjusted_point.0 - 1usize, adjusted_point.1);
                marked_visted(maze, &adjusted_point)
            },
            1 => {
                adjusted_point = (adjusted_point.0 + 1usize, adjusted_point.1);
                marked_visted(maze, &adjusted_point)
            },
            2 => {
                adjusted_point = (adjusted_point.0, adjusted_point.1 - 1usize);
                marked_visted(maze, &adjusted_point)
            },
            3 => {
                adjusted_point = (adjusted_point.0, adjusted_point.1 + 1usize);
                marked_visted(maze, &adjusted_point)
            },
            _ => ()
        }
        adjusted_point = point.clone();
    }
}

fn marked_visted(maze: &mut std::vec::Vec<std::vec::Vec<std::string::String>>, point: &(usize,usize)) {
    if maze[point.0][point.1] == "O" {
        maze[point.0][point.1] = "X".to_string()
    }
}


// fn create_lines(maze: &mut std::vec::Vec<std::vec::Vec<std::string::String>>, point: &(usize,usize),  game_output: &TerminalScreen) {
//     unimplemented!()
// }

/*
    // loop {
    //     if total_nodes == 0 {
    //         break;
    //     }
    // }

    // let mut total_nodes_lp: u64 = 0u64;
    //
    // for i in 0..game_output.height {
    //     for j in 0..game_output.width {
    //         let tester: &str = &maze[i][j];
    //         match tester {
    //             //"┌" | "└" | "┐" | "┘" | "─" | "│" => (),
    //             "O" => total_nodes_lp += 1,
    //             _ => ()
    //         }
    //     }
    // }
    // println!("Loop: {}", total_nodes_lp);
    // println!("X: {}", total_nodes_gm);
    // assert_eq!(total_nodes_lp,total_nodes_gm);
*/

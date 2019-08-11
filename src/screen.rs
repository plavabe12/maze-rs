// Copyright © 2019 Ebraheem AlAthari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// *** Rust Declarations ***
use terminal_size::{Width, Height, terminal_size};
use rand::{self, Rng};

// *** Structs ***
pub struct TerminalScreen {
    pub width: usize,
    pub height: usize
}

// *** Public Functions ***
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
    // Generate Starting Point
    // Min: (1,1)
    // Max: (game_output.height - 2usize,game_output.width - 2usize)
    generate_lines(maze,game_output);
    clean_maze(maze);
}

pub fn print_maze(maze: & Vec<Vec<String>>) {
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            print!("{}",maze[i][j]);
        }
        println!();
    }
}

// *** Private Functions ***
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
    //Marking four points nearby that have been visited
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
    //Marking a point visited with and X if it has an O
    if maze[point.0][point.1] == "O" {
        maze[point.0][point.1] = "X".to_string()
    }
}

fn generate_lines(maze: &mut std::vec::Vec<std::vec::Vec<std::string::String>>, game_output: &TerminalScreen) {
    //Generates random lines in the maze
    let mut loop_value: u64 = 0u64;
    let mut horizantal_point: Vec<usize> = Vec::new();
    let mut vertical_point: Vec<usize> = Vec::new();
    while loop_value != 20u64 {
        if rand::random() {
            let horizantal: usize = rand::thread_rng().gen_range(1, game_output.height - 2usize) as usize;
            if !find_similar_value(&horizantal_point,&horizantal)  {
                let end_point:  usize = rand::thread_rng().gen_range((game_output.width - 2usize) / 2, game_output.width - 2usize) as usize;
                let mut movable_point: usize = rand::thread_rng().gen_range(2, end_point / 2) as usize;
                while movable_point != end_point {
                    maze[horizantal][movable_point] = " ".to_string();
                    let move_point: (usize, usize) = (horizantal,movable_point);
                    visited(maze, &move_point);
                    movable_point += 1;
                }
                loop_value += 1;
                horizantal_point.push(horizantal);
            }
        } else {
            let vertical: usize = rand::thread_rng().gen_range(1, game_output.width - 2usize) as usize;
            if !find_similar_value(&vertical_point,&vertical) {
                let end_point:  usize = rand::thread_rng().gen_range((game_output.height - 2usize) / 2, game_output.height - 2usize) as usize;
                let mut movable_point: usize = rand::thread_rng().gen_range(2, end_point / 2) as usize;
                while movable_point != end_point {
                    maze[movable_point][vertical] = " ".to_string();
                    let move_point: (usize, usize) = (movable_point,vertical);
                    visited(maze, &move_point);
                    movable_point += 1;
                }
                loop_value += 1;
                vertical_point.push(vertical);
            }
        }
    }
}

fn clean_maze(maze: &mut std::vec::Vec<std::vec::Vec<std::string::String>>) {
    //Produces a cleaner maze output
    replace_string(maze, "O".to_string(), "X".to_string());
    replace_string(maze, " ".to_string(), "A".to_string());
    replace_string(maze, "X".to_string(), " ".to_string());
    replace_string(maze, "A".to_string(), "X".to_string());
}

fn replace_string(maze: &mut std::vec::Vec<std::vec::Vec<std::string::String>>, find_value: String, replace_value: String) {
    //Replaces strings in the maze with another string
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            let push_value = replace_value.clone();
            if maze[i][j] == find_value {
                maze[i][j] = push_value;
            }
        }
    }
}

fn find_similar_value(points: &Vec<usize>, point: &usize) -> bool {
    //Checks if the point exists in the list
    if points.contains(&(point - 1)) {
        true
    } else if points.contains(&(point + 1)) {
        true
    } else if points.contains(&point) {
        true
    } else {
        false
    }
}

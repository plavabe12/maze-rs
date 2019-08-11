// Copyright © 2019 Ebraheem AlAthari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// *** Rust Declarations ***
use std::{thread, time};
use crate::screen::{TerminalScreen, print_maze};
use rand::{self, Rng};
use crossterm_input::{input, InputEvent, KeyEvent, RawScreen};
use std::process::exit;

// *** Structs ***
struct Point {
    x: usize,
    y: usize
}

// *** Public Functions ***
pub fn play_game(maze: &mut Vec<Vec<String>>, game_output: &TerminalScreen) {
    let end_point: Point = generate_valid_point(maze, game_output);
    place_string(maze, &end_point, "E".to_string());
    thread::sleep(time::Duration::from_millis(20));
    let mut player_point: Point = generate_valid_point(maze, game_output);
    place_string(maze, &player_point, "P".to_string());
    println!("Maze Generated");
    print_maze(&maze);
    if let Ok(_raw) = RawScreen::into_raw_mode() {
        let input = input();

        let mut sync_stdin = input.read_sync();

        loop {
            let event = sync_stdin.next();

            if let Some(key_event) = event {
                let old_point: Point =  Point { x: player_point.x.clone() , y: player_point.y.clone() };
                let processed_input: (Point,bool) = process_input_event(key_event, maze, game_output, player_point);
                player_point = processed_input.0;
                let end_game = processed_input.1;
                if end_game {
                    println!("Exiting current maze\r");
                    println!("Generating New Maze in 5 seconds\r");
                    thread::sleep(time::Duration::from_millis(5000));
                    break;
                } else if player_point.x == end_point.x && player_point.y == end_point.y {
                    place_string(maze, &old_point, " ".to_string());
                    place_string(maze, &end_point, "P".to_string());
                    print_maze_in_raw_mode(&maze);
                    println!("You Won\r");
                    println!("Generating New Maze in 5 seconds\r");
                    thread::sleep(time::Duration::from_millis(5000));
                    break;
                } else {
                    place_string(maze, &old_point, " ".to_string());
                    place_string(maze, &player_point, "P".to_string());
                }
                print_maze_in_raw_mode(&maze);
            }
        }
    }
}

// *** Private Functions ***
fn generate_valid_point(maze: &Vec<Vec<String>>, game_output: &TerminalScreen) -> Point {
    let mut new_point: Point;
    loop {
        let point_1: usize = rand::thread_rng().gen_range(1, game_output.height - 2usize) as usize;
        let point_2: usize = rand::thread_rng().gen_range(1, game_output.width - 2usize) as usize;
        new_point = Point { x:point_1, y:point_2 };
        if valid_point(maze, game_output, &new_point) {
            break;
        }
    }
    new_point
}

fn valid_point(maze: &Vec<Vec<String>>, game_output: &TerminalScreen, point: &Point) -> bool {
    if point.x > game_output.height || point.y > game_output.width {
        false
    } else {
        if maze[point.x][point.y] == " " {
            true
        } else {
            false
        }
    }
}

fn place_string(maze: &mut Vec<Vec<String>>, point: &Point, value_to_place: String) {
    maze[point.x][point.y] = value_to_place;
}

fn exit_read() -> () {
    if let Ok(_) = RawScreen::disable_raw_mode() {
        exit(0x0100);
    }
}

//process_input_event(key_event, &mut player_point, &mut end_game);
fn process_input_event(key_event: InputEvent, maze: &Vec<Vec<String>>, game_output: &TerminalScreen, mut playerpoint: Point) -> (Point,bool) {
    let default_point: Point = Point { x: playerpoint.x.clone() , y: playerpoint.y.clone() };
    match key_event {
        InputEvent::Keyboard(k) => {
            match k {
                KeyEvent::Char(c) => match c {
                    'n' => {
                        (playerpoint,true)
                    }
                    'w' => {
                        playerpoint.x -= 1;
                        if valid_point_game_point(maze,game_output,&playerpoint) {
                            (playerpoint,false)
                        } else {
                            println!("Can't Move there\r");
                            (default_point,false)
                        }
                    }
                    'a' => {
                        playerpoint.y += 1;
                        if valid_point_game_point(maze,game_output,&playerpoint) {
                            (playerpoint,false)
                        } else {
                            println!("Can't Move there\r");
                            (default_point,false)
                        }
                    }
                    's' => {
                        playerpoint.x += 1;
                        if valid_point_game_point(maze,game_output,&playerpoint) {
                            (playerpoint,false)
                        } else {
                            println!("Can't Move there\r");
                            (default_point,false)
                        }
                    }
                    'd' => {
                        playerpoint.y -= 1;
                        if valid_point_game_point(maze,game_output,&playerpoint) {
                            (playerpoint,false)
                        } else {
                            println!("Can't Move there\r");
                            (default_point,false)
                        }
                    }
                    _ => (default_point,false)
                },
                KeyEvent::Esc => {
                    println!("The 'ESC' key is hit and the program is not listening to input anymore.\r");
                    exit_read();
                    (playerpoint,true)
                }
                KeyEvent::Up => {
                    playerpoint.x -= 1;
                    if valid_point_game_point(maze,game_output,&playerpoint) {
                        (playerpoint,false)
                    } else {
                        println!("Can't Move there\r");
                        (default_point,false)
                    }
                }
                KeyEvent::Right => {
                    playerpoint.y += 1;
                    if valid_point_game_point(maze,game_output,&playerpoint) {
                        (playerpoint,false)
                    } else {
                        println!("Can't Move there\r");
                        (default_point,false)
                    }
                }
                KeyEvent::Down => {
                    playerpoint.x += 1;
                    if valid_point_game_point(maze,game_output,&playerpoint) {
                        (playerpoint,false)
                    } else {
                        println!("Can't Move there\r");
                        (default_point,false)
                    }
                }
                KeyEvent::Left => {
                    playerpoint.y -= 1;
                    if valid_point_game_point(maze,game_output,&playerpoint) {
                        (playerpoint,false)
                    } else {
                        println!("Can't Move there\r");
                        (default_point,false)
                    }
                }
                _ => (default_point,false)
            }
        }
        _ => (default_point,false),
    }
}

fn print_maze_in_raw_mode(maze: & Vec<Vec<String>>) {
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            print!("{}",maze[i][j]);
        }
        println!("\r");
    }
}

fn valid_point_game_point(maze: &Vec<Vec<String>>, game_output: &TerminalScreen, point: &Point) -> bool {
    if point.x > game_output.height || point.y > game_output.width {
        false
    } else {
        if (maze[point.x][point.y] == " ") || (maze[point.x][point.y] == "E") {
            true
        } else {
            false
        }
    }
}

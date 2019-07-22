// Copyright © 2019 Ebraheem AlAthari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
mod screen;

struct Point {
    x: u64,
    y: u64
}

use std::process;
mod player;


use crossterm_input::{input, InputEvent, KeyEvent, RawScreen};

fn exit_read() -> () {
    if let Ok(_) = RawScreen::disable_raw_mode() {
        process::exit(0x0100);
    }
}

fn process_input_event(key_event: InputEvent, mut playerpoint: Point) -> (bool,Point) {
    match key_event {
        InputEvent::Keyboard(k) => {
            match k {
                KeyEvent::Char(c) => match c {
                    'q' => {
                        println!("The 'q' key is hit and the program is not listening to input anymore.\r");
                        exit_read();
                    }
                    'w' => {
                        playerpoint.x += 1;
                    }
                    'a' => {
                        playerpoint.y += 1;
                    }
                    's' => {
                        if playerpoint.x != 0 {
                            playerpoint.x -= 1;
                        }
                    }
                    'd' => {
                        if playerpoint.y != 0 {
                            playerpoint.y -= 1;
                        }
                    }
                    _ => ()
                },
                /*
                KeyEvent::Alt(c) => {
                    println!("{}", format!("ALT +'{}' pressed\r", c));
                }
                KeyEvent::Ctrl(c) => {
                    println!("{}", format!("CTRL +'{}' Pressed\r", c));
                } */
                KeyEvent::Esc => {
                    println!("{}", format!("ESC pressed and the program is not listening to input anymore.\r"));
                    exit_read();
                }
                KeyEvent::Up => {
                    playerpoint.x += 1;
                }
                KeyEvent::Right => {
                    playerpoint.y += 1;
                }
                KeyEvent::Down => {
                    if playerpoint.x != 0 {
                        playerpoint.x -= 1;
                    }
                }
                KeyEvent::Left => {
                    if playerpoint.y != 0 {
                        playerpoint.y -= 1;
                    }
                }
                _ => ()
            }
        }
        _ => (),
    }
    return (false,playerpoint);
}

pub fn read_synchronously() {
    // make sure to enable raw mode, this will make sure key events won't be handled by the terminal it's self and allows crossterm to read the input and pass it back to you.
    if let Ok(_raw) = RawScreen::into_raw_mode() {
        let input = input();

        let mut sync_stdin = input.read_sync();
        let mut playerpoint: Point = Point {x: 0, y:0};

        loop {
            let event = sync_stdin.next();

            if let Some(key_event) = event {
                let (to_cont, ret_playerpoint) = process_input_event(key_event,playerpoint);
                playerpoint = ret_playerpoint;
                if to_cont {
                    break;
                } else {
                    println!("Player coordinates: ({}, {})\r", playerpoint.x, playerpoint.y);
                }
            }
        }
    } // <=== raw modes will be disabled here
}

fn main() {
    screen::create_game_windows();
    //player::setup_player();
    //read_synchronously();
}

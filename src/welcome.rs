// Copyright © 2019 Ebraheem AlAthari
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// *** Rust Declarations ***
use std::{thread, time};

// *** Public Functions ***
pub fn welcome_msg() {
    let message1: String = "Welcome to the maze game".to_string();
    let message2: String = "You the player (\u{A9}) must find the exit (\u{24})".to_string();
    let message3: String = "You may use the arrow key or wasd to move around".to_string();
    let message4: String = "If a maze can't be completed press n".to_string();
    let message5: String = "to generate a new maze".to_string();
    let message6: String = "To leave the game press the ESC key".to_string();
    let message7: String = "Good Luck and Enjoy also ctrl c won't work".to_string();
    let message8: String = "Game starts in 30 seconds and there is a buffer".to_string();
    let message9: String = "of 5 seconds between new maze games".to_string();
    print!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        message1, message2, message3, message4, message5, message6, message7, message8, message9
    );
    thread::sleep(time::Duration::from_millis(30000));
}

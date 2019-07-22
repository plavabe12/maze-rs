use terminal_size::{Width, Height, terminal_size};

struct TerminalScreen {
    width: usize,
    height: usize
}

impl TerminalScreen {
    fn get_terminal_size(&mut self) {
        let size = terminal_size();

        if let Some((Width(w), Height(h))) = size {
            self.width = w as usize;
            self.height = h as usize;
        }
    }
}

pub fn create_game_windows() {
    let mut new_terminal: TerminalScreen = TerminalScreen { width: 0, height: 0};
    TerminalScreen::get_terminal_size(&mut new_terminal);
    println!("Terminal Size is {} {}",new_terminal.width,new_terminal.height);
}


    /*
    if let Some((Width(w), Height(h))) = size {
        let width_terminal = vec!['.'.to_string(); w as usize - 1usize];
        let print_screen = vec![width_terminal.clone(); h as usize - 1usize];
        for i in 0..print_screen.len() {
            for j in 0..print_screen[i].len() {
                if i == 0 && j == 0 {
                    print!("/");
                } else {
                    print!("{}", print_screen[i][j]);
                }

            }
            println!();
        }
    } else {
        false
    }*/

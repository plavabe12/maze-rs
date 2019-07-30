use terminal_size::{Width, Height, terminal_size};

// struct TerminalScreen {
//     width: usize,
//     height: usize
// }

pub fn get_terminal_size() -> (usize,usize) {
    if let Some((Width(w), Height(h))) = terminal_size() {
        (w as usize, h as usize)
    } else {
        (0usize, 0usize)
    }
}

// fn get_terminal_size(new_terminal: &mut TerminalScreen) {
//     let size = terminal_size();
//     if let Some((Width(w), Height(h))) = size {
//         new_terminal.width = w as usize - 2usize;
//         new_terminal.height = h as usize - 2usize;
//     }
// }

// fn build_game_screen(self) -> Vec<Vec<String>> {
//     let terminal_width: Vec<String> = vec!['.'.to_string(); self.width];
//     vec![terminal_width.clone(); self.height]
// }

// pub fn create_game_box() /* -> Vec<Vec<String>> */  {
//     let mut new_terminal: TerminalScreen = TerminalScreen { width: 0, height: 0};
//     get_terminal_size(&mut new_terminal);
//     println!("{} {}", new_terminal.height, new_terminal.width)
//     //TerminalScreen::build_game_screen(new_terminal)
// }

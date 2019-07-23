use terminal_size::{Width, Height, terminal_size};

struct TerminalScreen {
    width: usize,
    height: usize
}

impl TerminalScreen {
    fn get_terminal_size(&mut self) {
        let size = terminal_size();

        if let Some((Width(w), Height(h))) = size {
            self.width = w as usize - 2usize;
            self.height = h as usize - 2usize;
        }
    }

    fn build_game_screen(self) -> Vec<Vec<String>> {
        let terminal_width: Vec<String> = vec!['.'.to_string(); self.width];
        vec![terminal_width.clone(); self.height]
    }
}

pub fn create_game_box() -> Vec<Vec<String>>  {
    let mut new_terminal: TerminalScreen = TerminalScreen { width: 0, height: 0};
    TerminalScreen::get_terminal_size(&mut new_terminal);
    TerminalScreen::build_game_screen(new_terminal)
}

#[test]
fn test_vector_creation() {
    let new_terminal: TerminalScreen = TerminalScreen { width: 10, height: 10};
    let expected: Vec<Vec<String>> = vec![vec!['.'.to_string();10];10];
    assert_eq!(expected, TerminalScreen::build_game_screen(new_terminal))
}

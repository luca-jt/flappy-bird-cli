pub mod func_lib {

    use std::io::{self, Write};
    use std::time::Duration;
    use crossterm::event::{self, KeyCode, KeyEvent, Event};
    use rand::rngs::ThreadRng;
    use rand::Rng;


    pub const BOARD_STR: &'static str = r#"                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
"#;


    pub const FPS: u8 = 60;
    pub const PLAYER_JUMP_SPEED: u8 = 7;
    pub const G_FORCE: f32 = 9.81;
    pub const ANIMATION_SPEED: f32 = 2.5;

    pub const PLAYER_X: usize = 5;
    pub const LINE_LENGTH: usize = 64;
    pub const NUMBER_OF_LINES: usize = 24;
    pub const PLAYER_START_Y: usize = NUMBER_OF_LINES / 2;
    pub const GAP_SIZE: usize = 6;
    pub const PIXELS_BETWEEN_COLS: u16 = 16;
    pub const SECONDS_BETWEEN_SHIFTS: f32 = 0.2;


    pub fn space_pressed() -> bool
    {
        loop
        {
            if event::poll(Duration::from_millis(10)).unwrap() {
                if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                    if code == KeyCode::Char(' ').into() {
                        return true;
                    }
                }
            }
            break;
        }
        false
    }


    pub fn calc_player_pos(t: f32, v0: f32, h: i16) -> i16
    {
        (0.5 * G_FORCE * t*t - v0 * t) as i16 + h
    }


    pub fn set_player_pos(new_y: i16, former_y: i16, board: &mut Vec<Vec<char>>) -> bool
    {
        if new_y > NUMBER_OF_LINES as i16 - 1 || new_y < 0 {
            return false;
        }
        board[former_y as usize][PLAYER_X] = ' ';
        if board[new_y as usize][PLAYER_X] == '#' {
            return false;
            // TODO: check for '#' inbetween to avoid jumping over them
        }
        board[new_y as usize][PLAYER_X] = '@';

        true
    }


    pub fn shift_cols(board: &mut Vec<Vec<char>>)
    {
        for y in 0..NUMBER_OF_LINES {
            for x in 0..LINE_LENGTH {
                if board[y][x] == '#' {
                    board[y][x] = ' ';
                    if x > 0 {
                        board[y][x - 1] = '#';
                    }
                }
            }
        }
    }


    pub fn check_if_col_passed(board: &Vec<Vec<char>>) -> bool
    {
        board[0][PLAYER_X] == '#' ||
        board[NUMBER_OF_LINES - 1][PLAYER_X] == '#'
    }


    pub fn draw_new_col(board: &mut Vec<Vec<char>>, rng: &mut ThreadRng, gap_size: usize)
    {
        let random_upper_bound = rng.gen_range(2..=NUMBER_OF_LINES - 2 - gap_size);
        
        for y in 0..random_upper_bound {
            board[y][LINE_LENGTH - 1] = '#';
        }
        for y in random_upper_bound + gap_size..NUMBER_OF_LINES {
            board[y][LINE_LENGTH - 1] = '#';
        }
    }


    pub fn print_board(board: &Vec<Vec<char>>, score: u16)
    {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();

        println!("SCORE: {}", score);
        println!("------------------------------------------------------------------");
        for v in board {
            let row_string: String = v.iter().collect();
            println!("|{}|", row_string);
        }
        println!("------------------------------------------------------------------")
    }


}

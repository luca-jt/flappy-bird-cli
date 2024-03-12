pub mod func_lib {

    use std::io::{self, Write, Read, stdin, stdout};
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


    pub fn shift_cols(board: &mut Vec<Vec<char>>, changed_chars: &mut Vec<Vec<u16>>)
    {
        for y in 0..NUMBER_OF_LINES {
            for x in 0..LINE_LENGTH {
                if board[y][x] == '#' {
                    board[y][x] = ' ';
                    if x > 0 {
                        board[y][x - 1] = '#';
                        changed_chars.push(vec![x as u16, y as u16]);
                    }
                    if x > 1 {
                        changed_chars.push(vec![x as u16 - 1, y as u16]);
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


    pub fn print_board(board: &Vec<Vec<char>>, score: u16, speed: f32)
    {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();

        println!("SCORE: {:3} | SPEED: {:.3}", score, speed);
        println!("------------------------------------------------------------------");
        for v in board {
            let row_string: String = v.iter().collect();
            println!("|{}|", row_string);
        }
        println!("------------------------------------------------------------------")
    }


    pub fn pause()
    {
        let mut out = stdout();
        out.write(b"Press Enter to quit...").unwrap();
        out.flush().unwrap();
        stdin().read(&mut [0]).unwrap();
    }


    fn set_cursor_position(row: u16, col: u16)
    {
        print!("\x1B[{};{}H", row, col);
        io::stdout().flush().unwrap();
    }


    fn delete_number_chars(num: usize)
    {
        for _ in 0..num
        {
            print!("\x08");
        }
        io::stdout().flush().unwrap();
    }


    pub fn edit_board(board: &mut Vec<Vec<char>>, score: u16, speed: f32, changed_chars: &mut Vec<Vec<u16>>)
    {
        set_cursor_position(1, 11);
        delete_number_chars(3);
        print!("{:3}", score);
        set_cursor_position(1, 26);
        delete_number_chars(5);
        print!("{:.3}", speed);

        for pair in changed_chars.clone()
        {
            if let Some([x, y, ..]) = <&[u16] as TryInto<&[u16]>>::try_into(pair.as_slice()).ok() {
                set_cursor_position(y+3, x+2);
                delete_number_chars(1);
                print!("{}", board[*y as usize][*x as usize]);
            }
        }

        io::stdout().flush().unwrap();
        changed_chars.clear();
    }

}

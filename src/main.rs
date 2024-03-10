pub mod framerate_capper;
use std::cmp::min;
use std::io::{self, Write};
use std::time::Duration;
use crossterm::event::{self, KeyCode, KeyEvent, Event};
pub use crate::framerate_capper::fps_capping::FpsCapper;


const BOARD_STR: &'static str = r#"                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
                                                                
"#;


const FPS: u8 = 10;
const PLAYER_JUMP_SPEED: u8 = 9;
const G_FORCE: f32 = 9.81;
const ANIMATION_SPEED: f32 = 1.0;

const PLAYER_X: usize = 3;
const LINE_LENGTH: usize = 64;
const NUMBER_OF_LINES: usize = 24;
const PLAYER_START_Y: usize = NUMBER_OF_LINES / 2;


fn space_pressed() -> bool
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


fn calc_player_pos(t: f32, v0: f32, h: i16) -> i16
{
    (0.5 * G_FORCE * t*t - v0 * t) as i16 + h
}


fn set_player_pos(new_y: i16, former_y: i16, board: &mut Vec<Vec<char>>) -> bool
{
    if new_y > NUMBER_OF_LINES as i16 - 1 || new_y < 0 {
        return false;
    }
    board[former_y as usize][PLAYER_X] = ' ';
    if board[new_y as usize][PLAYER_X] == '#' {
        return false;
    }
    board[new_y as usize][PLAYER_X] = '@';
    true
}


fn shift_cols(board: &mut Vec<Vec<char>>)
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


fn check_if_col_passed(board: &Vec<Vec<char>>) -> bool
{
    board[0][PLAYER_X] == '#' ||
    board[NUMBER_OF_LINES - 1][PLAYER_X] == '#'
}


fn print_board(board: &Vec<Vec<char>>, score: u16)
{
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();

    println!("SCORE: {}", score);
    println!("----------------------------------------------------------------");
    for v in board {
        let row_string: String = v.iter().collect();
        println!("{}", row_string);
    }
    println!("----------------------------------------------------------------")
}


fn main()
{
    let pre_board: Vec<String> = BOARD_STR.split('\n')
                                          .map(String::from)
                                          .collect();

    let mut board: Vec<Vec<char>> = pre_board.iter()
                                             .map(|s| s.chars().collect())
                                             .collect();

    let mut score: u16 = 0;
    let mut loop_counter: u16 = 0;
    let mut shift_speed: u8 = 1;
    let mut loops_since_keypress: u16 = 0;
    let mut fps_capper = FpsCapper::new(FPS);

    let mut current_y = PLAYER_START_Y;
    board[current_y][PLAYER_X] = '@';
    let mut last_jump_y = current_y;

    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
    print_board(&board, score);
    while !space_pressed() {
        FpsCapper::start_measurement(&mut fps_capper);
        FpsCapper::cap_fps(&mut fps_capper);
    }

    let mut running = true;
    while running
    {
        FpsCapper::start_measurement(&mut fps_capper);

        print_board(&board, score);
        
        let new_y = calc_player_pos((loops_since_keypress / FPS as u16) as f32 * ANIMATION_SPEED,
                                        PLAYER_JUMP_SPEED as f32,
                                        last_jump_y as i16);

        running = set_player_pos(new_y, current_y as i16, &mut board); 
        current_y = new_y as usize;
        
        if check_if_col_passed(&board) {
            score += 1;
        }
        if space_pressed() {
            last_jump_y = current_y;
            loops_since_keypress = 0;
        }

        loop_counter %= (FPS / shift_speed) as u16;
        if loop_counter == 0 {
            shift_cols(&mut board);
        }
        if score % min(10, FPS as u16) == 0 && shift_speed < 10 && score != 0 {
            shift_speed += 1
        }

        loops_since_keypress += 1;
        loop_counter += 1;
        
        FpsCapper::cap_fps(&mut fps_capper);
    }

    println!("FINAL SCORE: {}", score);
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

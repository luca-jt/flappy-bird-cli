
pub mod framerate_capper;
use std::io::{self, Write};
use std::time::Duration;
use crossterm::event::{self, KeyCode, KeyEvent, Event};

pub use crate::framerate_capper::fps_capping::FpsCapper;


const BOARD_STR: &'static str = r#"
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
"#;

const FPS: u8 = 30;
const PLAYER_X: usize = 3;
const PLAYER_JUMP_SPEED: u8 = 8;
const LINE_LENGTH: usize = 64 + 1;
const NUMBER_OF_LINES: usize = BOARD_STR.len() / LINE_LENGTH;
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




fn calc_player_pos(t: f32, v0: f32, h: i8) -> i8
{
    (0.5 * 9.81 * t*t - v0 * t) as i8 + h
}


fn set_player_pos(new_y: i8, former_y: i8, board: &mut Vec<char>) -> bool
{
    if new_y > NUMBER_OF_LINES as i8 - 1 || new_y < 0 {
        return false;
    }
    if let Some(changed_char) = board.get_mut(former_y as usize * LINE_LENGTH + PLAYER_X) {
        *changed_char = ' '
    }
    if board.get(new_y as usize * LINE_LENGTH + PLAYER_X).unwrap() == &'#' {
        return false;
    }
    if let Some(changed_char) = board.get_mut(new_y as usize * LINE_LENGTH + PLAYER_X) {
        *changed_char = '@'
    }
    true
}


fn shift_cols(board: &mut Vec<char>)
{
    for i in 0..board.len() {
        if board.get(i).unwrap() == &'#' {
            board[i] = ' ';
            if i & LINE_LENGTH > 0 {
                board[i - 1] = '#';
            }
        }
    }
}


fn check_if_col_passed(board: &Vec<char>) -> bool
{
    board.get(PLAYER_X).unwrap() == &'#' ||
    board.get(PLAYER_X + LINE_LENGTH * (NUMBER_OF_LINES - 1)).unwrap() == &'#'
}


fn print_board(board: &Vec<char>, score: u16)
{
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
    println!("SCORE: {}", score);
    let board_string: String = board.iter().collect();
    println!("{}", board_string);
}


fn main()
{
    let mut board: Vec<char> = BOARD_STR.chars().collect();

    let mut score: u16 = 0;
    let mut loop_counter: u16 = 0;
    let mut shift_speed: u8 = 1;
    let mut loops_since_keypress: u16 = 0;
    let mut fps_capper: FpsCapper = FpsCapper::new(FPS);

    let mut current_y = PLAYER_START_Y;
    if let Some(changed_char) = board.get_mut(current_y * LINE_LENGTH + PLAYER_X) {
        *changed_char = '@'
    }
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
        
        let new_y = calc_player_pos((loops_since_keypress / FPS as u16) as f32, PLAYER_JUMP_SPEED as f32, last_jump_y as i8);
        running = set_player_pos(new_y, current_y as i8, &mut board); 
        current_y = new_y as usize;
        if check_if_col_passed(&board) {
            score += 1
        }
        if space_pressed() {
            last_jump_y = current_y;
            loops_since_keypress = 0;
        }

        loop_counter %= (FPS / shift_speed) as u16;
        if loop_counter == 0 {
            shift_cols(&mut board)
        }
        if score % 10 == 0 && shift_speed < 10 {
            shift_speed += 1
        }

        loops_since_keypress += 1;
        loop_counter += 1;
        
        FpsCapper::cap_fps(&mut fps_capper);
    }

    println!("FINAL SCORE: {}", score);
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

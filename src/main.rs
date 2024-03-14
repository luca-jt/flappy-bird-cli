pub mod framerate_capper;
pub mod func_lib;
use std::cmp::min;
use crossterm::{cursor, execute};
use std::io::stdout;
pub use crate::framerate_capper::fps_capping::FpsCapper;
pub use crate::func_lib::func_lib::*;
 

fn main()
{
    let mut board: Vec<Vec<char>> = vec![vec![' '; LINE_LENGTH]; NUMBER_OF_LINES];
    
    let mut score: u16 = 0;
    let mut loop_counter: u16 = 0;
    let mut shift_counter: u16 = 0;
    let mut shift_speed: f32 = 1.0;
    let mut loops_since_keypress: u16 = 0;
    let mut fps_capper = FpsCapper::new(FPS);
    let mut rng = rand::thread_rng();

    let mut current_y = PLAYER_START_Y;
    board[current_y][PLAYER_X] = '@';
    let mut last_jump_y = current_y;

    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
    execute!(stdout(),cursor::Hide).unwrap(); // hide cursor
    print_board(&board, score, shift_speed);
    print!("\x1B[s"); // save end-of-board cursor position
    while !space_pressed() {
        FpsCapper::start_measurement(&mut fps_capper);
        FpsCapper::cap_fps(&mut fps_capper);
    }

    let mut frame_changed = true;
    let mut changed_chars: Vec<Vec<u16>> = vec![vec![PLAYER_X as u16 + 1, current_y as u16]]; // Vec<[x: u16, y: u16]>
    let mut drew = false;
    let mut running = true;
    while running
    {
        FpsCapper::start_measurement(&mut fps_capper);

        if frame_changed {
            edit_board(&mut board, score, shift_speed, &mut changed_chars);
            frame_changed = false;
        }
        
        let new_y = calc_player_pos(loops_since_keypress as f32 / FPS as f32 * ANIMATION_SPEED,
                                        PLAYER_JUMP_SPEED as f32,
                                        last_jump_y as i16);

        running = set_player_pos(new_y, current_y as i16, &mut board); 
        if current_y != new_y as usize {
            changed_chars.push(vec![PLAYER_X as u16, current_y as u16]);
            changed_chars.push(vec![PLAYER_X as u16, new_y as u16]);
            current_y = new_y as usize;
            frame_changed = true;
        }
        
        if space_pressed() {
            last_jump_y = current_y;
            loops_since_keypress = 0;
            frame_changed = true;
        }
 
        if shift_counter % PIXELS_BETWEEN_COLS == 0 && !drew {
            draw_new_col(&mut board, &mut rng, GAP_SIZE);
            frame_changed = true;
            drew = true;
        }

        if loop_counter % (FPS as f32 * SECONDS_BETWEEN_SHIFTS / shift_speed) as u16 == 0 {
            shift_cols(&mut board, &mut changed_chars);
            shift_counter = shift_counter.wrapping_add(1);

            if check_if_col_passed(&board) {
                score += 1;

                if score % min(5, FPS as u16) == 0 && shift_speed < 5.0 && score != 0 {
                    shift_speed += 0.005 * score as f32;
                }
            }

            frame_changed = true;
            drew = false;
        }

        loops_since_keypress += 1;
        loop_counter = loop_counter.wrapping_add(1);
        
        FpsCapper::cap_fps(&mut fps_capper);
    }

    print!("\x1B[u"); // restore cursor position
    execute!(stdout(), cursor::Show).unwrap(); // reveal cursor
    println!("FINAL SCORE: {}", score);
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    pause();
}

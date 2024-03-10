pub mod framerate_capper;
pub mod func_lib;
use std::cmp::min;
pub use crate::framerate_capper::fps_capping::FpsCapper;
pub use crate::func_lib::func_lib::*;


fn main()
{
    let mut pre_board: Vec<String> = BOARD_STR.split('\n').map(String::from).collect();
    let _ = pre_board.pop().unwrap();
    let mut board: Vec<Vec<char>> = pre_board.iter().map(|s| s.chars().collect()).collect();

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
    print_board(&board, score);
    while !space_pressed() {
        FpsCapper::start_measurement(&mut fps_capper);
        FpsCapper::cap_fps(&mut fps_capper);
    }

    let mut frame_changed = true;
    let mut drew = false;
    let mut running = true;
    while running
    {
        FpsCapper::start_measurement(&mut fps_capper);

        if frame_changed {
            print_board(&board, score);
            frame_changed = false;
        }
        
        let new_y = calc_player_pos(loops_since_keypress as f32 / FPS as f32 * ANIMATION_SPEED,
                                        PLAYER_JUMP_SPEED as f32,
                                        last_jump_y as i16);

        running = set_player_pos(new_y, current_y as i16, &mut board); 
        if current_y != new_y as usize {
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
            shift_cols(&mut board);
            shift_counter = shift_counter.wrapping_add(1);

            if check_if_col_passed(&board) {
                score += 1;
            }
            frame_changed = true;
            drew = false;
        }

        if score % min(5, FPS as u16) == 0 && shift_speed < 5.0 && score != 0 {
            shift_speed *= 1.002;
        }

        loops_since_keypress += 1;
        loop_counter = loop_counter.wrapping_add(1);
        
        FpsCapper::cap_fps(&mut fps_capper);
    }

    println!("FINAL SCORE: {}", score);
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

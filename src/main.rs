
pub mod framerate_capper;
pub use crate::framerate_capper::fps_capping::FpsCapper;


const BOARD_STR: &'static str = r#"
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
                                                               
"#;

const FPS: u8 = 30;
const PLAYER_X: usize = 3;
const PLAYER_JUMP_SPEED: u8 = 8;
const LINE_LENGTH: usize = 64 + 1;
const NUMBER_OF_LINES: usize = BOARD_STR.len() / LINE_LENGTH;
const PLAYER_START_Y: usize = NUMBER_OF_LINES / 2;


fn main()
{
    let mut board: Vec<char> = BOARD_STR.chars().collect();

    let mut score: u16 = 0;
    let mut loop_counter: u16 = 0;
    let mut shift_speed: u8 = 1;
    let mut loops_since_keypress: u16 = 0;
    let mut fps_capper: FpsCapper = FpsCapper::new(FPS);

    let current_y = PLAYER_START_Y;
    if let Some(changed_char) = board.get_mut(current_y * LINE_LENGTH + PLAYER_X) {
        *changed_char = '@'
    }
    let mut last_jump_y = current_y;

    //...

    let mut running = true;
    while running
    {
        FpsCapper::start_measurement(&mut fps_capper);

        //...

        loops_since_keypress += 1;
        loop_counter += 1;
        
        FpsCapper::cap_fps(&mut fps_capper);
    }

    println!("FINAL SCORE: {}", score);
}

pub mod fps_capping
{

    use std::time::{Instant, Duration};


    pub struct FpsCapper
    {
        start_time: Option<Instant>,
        frame_time: Duration
    }

    impl FpsCapper
    {
        pub fn new(fps: u8) -> FpsCapper
        {
            FpsCapper {
                start_time: None,
                frame_time: Duration::from_secs_f64(1.0 / f64::from(fps))
            }
        }

        pub fn start_measurement(fps_capper: &mut FpsCapper)
        {
            fps_capper.start_time = Some(Instant::now());
        }

        pub fn cap_fps(fps_capper: &mut FpsCapper)
        {
            let measured_time = fps_capper.start_time.unwrap().elapsed();
            if measured_time < fps_capper.frame_time
            {
                std::thread::sleep(fps_capper.frame_time - measured_time);
            }
        }
    }

}

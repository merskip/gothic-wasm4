use windows::Win32::System::SystemInformation::GetTickCount64;

pub struct FPSCounter {
    last_tick_count: u64,
    frame_count: u32,
    fps: u32,
}

impl FPSCounter {
    pub fn new() -> Self {
        FPSCounter {
            last_tick_count: Self::get_tick_count(),
            frame_count: 0,
            fps: 0,
        }
    }

    pub fn tick(&mut self) -> u32 {
        self.frame_count += 1;
        let current_tick_count = Self::get_tick_count();
        let delta_ticks = current_tick_count - self.last_tick_count;

        if delta_ticks >= 1000 {
            self.fps = (self.frame_count * 1000) / delta_ticks as u32;
            self.frame_count = 0;
            self.last_tick_count = current_tick_count;
        }

        self.fps
    }

    fn get_tick_count() -> u64 {
        unsafe { GetTickCount64() }
    }
}

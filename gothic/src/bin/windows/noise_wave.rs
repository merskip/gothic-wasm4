use std::cell::RefCell;
use std::time::Duration;
use rand::Rng;
use rand::rngs::ThreadRng;

use rodio::Source;

pub struct NoiseWave {
    freq: f32,
    rng: RefCell<ThreadRng>,
}

unsafe impl Send for NoiseWave {

}

impl NoiseWave {
    pub fn new(freq: f32) -> NoiseWave {
        NoiseWave {
            freq,
            rng: RefCell::new(rand::thread_rng()),
        }
    }
}

impl Iterator for NoiseWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let mut rng = self.rng.get_mut();
        let random = rng.gen::<f32>() * 2.0 - 1.0;
        Some(random)
    }
}

impl Source for NoiseWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.freq as u32 * 2
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

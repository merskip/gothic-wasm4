use std::ops::BitAnd;
use std::time::Duration;

use rodio::Source;

/// Implementation from https://github.com/aduros/wasm4/blob/main/runtimes/native/src/apu.c
pub struct NoiseWave {
    frequency: f32,
    phase: f32,
    seed: u16,
    last_random: f32,
}

const SAMPLE_RATE: f32 = 44_100.0;

impl NoiseWave {
    pub fn new(freq: f32) -> NoiseWave {
        NoiseWave {
            frequency: freq,
            phase: 0.0,
            seed: 0x0001,
            last_random: 0.0,
        }
    }
}

impl Iterator for NoiseWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.phase += self.frequency * self.frequency / (1_000_000.0 / SAMPLE_RATE * SAMPLE_RATE);
        while self.phase > 0.0 {
            self.phase -= 1.0;
            self.seed ^= self.seed >> 7;
            self.seed ^= self.seed << 9;
            self.seed ^= self.seed >> 13;
            self.last_random = (self.seed.bitand(0x1) as f32) * 2.0 - 1.0;
        }
        Some(self.last_random)
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
        SAMPLE_RATE as u32
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

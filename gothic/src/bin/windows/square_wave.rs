use std::f32::consts::PI;
use std::ops::Rem;
use std::time::Duration;
use rodio::Source;

/// An infinite source that produces a square wave.
/// Always has a rate of 48kHz and one channel.
pub struct SquareWave {
    freq: f32,
    num_sample: usize,
}

impl SquareWave {
    pub fn new(freq: f32) -> SquareWave {
        SquareWave {
            freq,
            num_sample: 0,
        }
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        // let value = (self.num_sample as f32).rem(48_000.0) / 48_000.0;
        // let sin = value.sin();
        // println!("value={value}");
        return if let 0 = self.num_sample % 2 {
            Some(1.0)
        } else {
            Some(-1.0)
        }
    }
}

impl Source for SquareWave {
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

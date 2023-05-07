use std::time::Duration;
use rodio::Source;

/// An infinite source that produces a square wave.
/// Always has a rate of 44.1KHz and one channel.
pub struct SquareWave {
    frequency: f32,
    fill_factor: f32,
    num_sample: usize,
}

const SAMPLE_RATE: f32 = 44_100.0;

impl SquareWave {
    pub fn new(frequency: f32, fill_factor: f32) -> SquareWave {
        SquareWave {
            frequency,
            fill_factor,
            num_sample: 0,
        }
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value = self.frequency * self.num_sample as f32 % SAMPLE_RATE / SAMPLE_RATE;
        return if value > self.fill_factor {
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
        SAMPLE_RATE as u32
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

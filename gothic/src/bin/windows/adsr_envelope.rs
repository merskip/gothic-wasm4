use std::time::Duration;

use rodio::{Sample, Source};

pub struct ADSREnvelope<I> {
    input: I,
    num_sample: usize,
    start_decay: usize,
    start_sustain: usize,
    start_release: usize,
    end_release: usize,
    peak_volume: f32,
    sustain_volume: f32,
    total_duration: Duration,
}

pub trait SourceADSREnvelope {
    fn adsr_envelope(
        self,
        attack: Duration,
        decay: Duration,
        sustain: Duration,
        release: Duration,
        peak_volume: f32,
        sustain_volume: f32,
    ) -> ADSREnvelope<Self> where Self: Sized, Self: Source, Self::Item: Sample;
}

impl<T> SourceADSREnvelope for T
    where T: Source, <T as Iterator>::Item: Sample {
    #[inline]
    fn adsr_envelope(
        self,
        attack: Duration,
        decay: Duration,
        sustain: Duration,
        release: Duration,
        peak_volume: f32,
        sustain_volume: f32,
    ) -> ADSREnvelope<Self> where Self: Sized, Self: Source {
        let sample_rate = self.sample_rate() as f32;
        let attack_duration = (sample_rate * attack.as_secs_f32()) as usize;
        let decay_duration = (sample_rate * decay.as_secs_f32()) as usize;
        let sustain_duration = (sample_rate * sustain.as_secs_f32()) as usize;
        let release_duration = (sample_rate * release.as_secs_f32()) as usize;
        ADSREnvelope {
            input: self,
            num_sample: 0,
            start_decay: attack_duration,
            start_sustain: attack_duration + decay_duration,
            start_release: sustain_duration + decay_duration + sustain_duration,
            end_release: sustain_duration + decay_duration + sustain_duration + release_duration,
            peak_volume,
            sustain_volume,
            total_duration: attack + decay + sustain + release,
        }
    }
}

impl<I> Iterator for ADSREnvelope<I>
    where I: Iterator,
          <I as Iterator>::Item: Sample
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.num_sample = self.num_sample.wrapping_add(1);
        if self.num_sample > self.end_release {
            return None;
        }

        return if let Some(value) = self.input.next() {
            let phase = self.current_phase();
            let volume = match phase {
                ADSRPhase::Attack(progress) =>
                    linear_interpolation(0.0, self.peak_volume, progress),

                ADSRPhase::Decay(progress) =>
                    linear_interpolation(self.peak_volume, self.sustain_volume, progress),

                ADSRPhase::Sustain =>
                    self.sustain_volume,

                ADSRPhase::Release(progress) =>
                    linear_interpolation(self.sustain_volume, 0.0, progress),
            };

            let new_value = value.amplify(volume);
            Some(new_value)
        } else {
            None
        };
    }
}

impl<I> Source for ADSREnvelope<I>
    where
        I: Iterator + Source,
        I::Item: Sample,
        <I as Iterator>::Item: Sample
{
    fn current_frame_len(&self) -> Option<usize> {
        return if self.num_sample <= self.end_release {
            Some(self.end_release - self.num_sample)
        } else {
            Some(0)
        };
    }

    fn channels(&self) -> u16 {
        self.input.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.input.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(self.total_duration)
    }
}

enum ADSRPhase {
    Attack(f32),
    Decay(f32),
    Sustain,
    Release(f32),
}

impl<I> ADSREnvelope<I> {
    fn current_phase(&self) -> ADSRPhase {
        return if self.num_sample < self.start_decay {
            ADSRPhase::Attack(
                progress(0, self.start_decay, self.num_sample)
            )
        } else if self.num_sample >= self.start_decay && self.num_sample < self.start_sustain {
            ADSRPhase::Decay(
                progress(self.start_decay, self.start_sustain, self.num_sample),
            )
        } else if self.num_sample >= self.start_sustain && self.num_sample < self.start_release {
            ADSRPhase::Sustain
        } else {
            ADSRPhase::Release(
                progress(self.start_release, self.end_release, self.num_sample),
            )
        };
    }
}

fn progress(from: usize, to: usize, value: usize) -> f32 {
    let progress = (value - from) as f32 / (to - from) as f32;
    progress.clamp(0.0, 1.0)
}

fn linear_interpolation(value1: f32, value2: f32, t: f32) -> f32 {
    value1 + (value2 - value1) * t
}

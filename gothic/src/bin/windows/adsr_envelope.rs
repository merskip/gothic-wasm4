use std::time::Duration;

use rodio::{Sample, Source};

pub struct ADSREnvelope<I> {
    input: I,
    attack: Duration,
    decay: Duration,
    sustain: Duration,
    release: Duration,
}

pub trait SourceADSREnvelope {
    fn adsr_envelope(
        self,
        attack: Duration,
        decay: Duration,
        sustain: Duration,
        release: Duration,
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
    ) -> ADSREnvelope<Self> where Self: Sized, Self: Source {
        ADSREnvelope {
            input: self,
            attack,
            decay,
            sustain,
            release,
        }
    }
}

impl<I> Iterator for ADSREnvelope<I>
    where I: Iterator {
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.next()
    }
}

impl<I> Source for ADSREnvelope<I>
    where
        I: Iterator + Source,
        I::Item: Sample,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.input.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.input.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.input.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.input.total_duration()
    }
}
use std::convert::TryFrom;

use libfvad_sys as ffi;

pub struct Fvad {
    fvad: *mut ffi::Fvad,
}

impl From<*mut ffi::Fvad> for Fvad {
    fn from(fvad: *mut ffi::Fvad) -> Self {
        Fvad { fvad }
    }
}

impl Drop for Fvad {
    fn drop(&mut self) {
        unsafe { ffi::fvad_free(self.fvad) }
    }
}

#[derive(Debug)]
pub enum Mode {
    Quality = 0,
    LowBitrate = 1,
    Aggressive = 2,
    VeryAggressive = 3,
}

#[derive(Debug)]
pub enum SampleRate {
    Rate8kHz = 8000,
    Rate16kHz = 16000,
    Rate32kHz = 32000,
    Rate48kHz = 48000,
}

impl TryFrom<u16> for SampleRate {
    type Error = ();

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        match n {
            8000 => Ok(SampleRate::Rate8kHz),
            16000 => Ok(SampleRate::Rate16kHz),
            32000 => Ok(SampleRate::Rate32kHz),
            48000 => Ok(SampleRate::Rate48kHz),
            _ => Err(()),
        }
    }
}

impl Fvad {
    /// Creates a VAD instance.
    pub fn new() -> Option<Self> {
        let fvad = unsafe { ffi::fvad_new() };
        match fvad.is_null() {
            false => Some(fvad.into()),
            true => None,
        }
    }

    /// Reinitializes a VAD instance, clearing all state and resetting mode and
    /// sample rate to defaults.
    pub fn reset(&mut self) {
        unsafe { ffi::fvad_reset(self.fvad) }
    }

    /// Changes the VAD operating ("aggressiveness") mode of a VAD instance.
    ///
    /// A more aggressive (higher mode) VAD is more restrictive in reporting speech.
    /// Put in other words the probability of being speech when the VAD returns 1 is
    /// increased with increasing mode. As a consequence also the missed detection
    /// rate goes up.
    pub fn set_mode(&mut self, mode: Mode) -> Option<()> {
        match unsafe { ffi::fvad_set_mode(self.fvad, mode as i32) } {
            -1 => None,
            0 => Some(()),
            n => panic!("fvad_set_mode() returned {}", n),
        }
    }

    /// Sets the input sample rate in Hz for a VAD instance.
    ///
    /// Note:
    /// that internally all processing will be done 8000 Hz; input data in higher
    /// sample rates will just be downsampled first.
    pub fn set_sample_rate(&mut self, sample_rate: SampleRate) -> Option<()> {
        match unsafe { ffi::fvad_set_sample_rate(self.fvad, sample_rate as i32) } {
            -1 => None,
            0 => Some(()),
            n => panic!("fvad_set_sample_rate() returned {}", n),
        }
    }

    /// Calculates a VAD decision for an audio frame.
    ///
    /// `frame` is a slice of signed 16-bit samples. Only slices with a
    /// length of 10, 20 or 30 ms are supported, so for example at 8 kHz, `frame.len()`
    /// must be either 80, 160 or 240.
    ///
    /// Returns              : Some(true) - (active voice),
    ///                       Some(false) - (non-active Voice),
    ///                       None - (invalid frame length).
    pub fn is_voice_frame(&mut self, frame: &[i16]) -> Option<bool> {
        match unsafe { ffi::fvad_process(self.fvad, frame.as_ptr(), frame.len() as u64) } {
            -1 => None,
            0 => Some(false),
            1 => Some(true),
            n => panic!("fvad_process() returned {}", n),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_sample_rate() {
        let mut vad = Fvad::new().unwrap();
        assert_eq!(
            vad.set_sample_rate(SampleRate::try_from(8000).unwrap()),
            Some(())
        );
        assert_eq!(vad.set_sample_rate(SampleRate::Rate8kHz), Some(()));
    }

    #[test]
    fn is_voice_frame() {
        let mut vad = Fvad::new().unwrap();

        let buffer = std::iter::repeat(0).take(160).collect::<Vec<i16>>();
        assert_eq!(vad.is_voice_frame(buffer.as_slice()), Some(false));
    }

    #[test]
    fn set_mode() {
        let mut vad = Fvad::new().unwrap();

        assert_eq!(vad.set_mode(Mode::Quality), Some(()));
    }
}

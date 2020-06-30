use std::convert::TryFrom;

use libfvad_sys as ffi;

#[derive(PartialEq, Debug)]
pub enum Mode {
    Quality = 0,
    LowBitrate = 1,
    Aggressive = 2,
    VeryAggressive = 3,
}

#[derive(PartialEq, Debug)]
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

pub struct Fvad {
    fvad: *mut ffi::Fvad,
}

impl Fvad {
    /// Creates and initializes a VAD instance.
    ///
    /// Returns:
    /// - Some(Self) wrapping a pointer to the new VAD instance on success
    /// - None in case of a memory allocation error
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
    /// The default mode is Mode::Quality.
    pub fn set_mode(&mut self, mode: Mode) -> &mut Self {
        let v = mode as i32;
        match unsafe { ffi::fvad_set_mode(self.fvad, v) } {
            0 => self,
            -1 => panic!("fvad_set_mode() did not accept {} as a valid mode", v),
            n => panic!("fvad_set_mode() returned {}", n),
        }
    }

    /// Sets the input sample rate in Hz for a VAD instance.
    ///
    /// Note that internally all processing will be done 8000 Hz; input data in higher
    /// sample rates will just be downsampled first.
    pub fn set_sample_rate(&mut self, sample_rate: SampleRate) -> &mut Self {
        let v = sample_rate as i32;
        match unsafe { ffi::fvad_set_sample_rate(self.fvad, v) } {
            0 => self,
            -1 => panic!(
                "fvad_set_sample_rate() did not accept {} as a valid sample_rate",
                v
            ),
            n => panic!("fvad_set_sample_rate() returned {}", n),
        }
    }

    /// Calculates a VAD decision for an audio frame.
    ///
    /// `frame` is a slice of signed 16-bit samples. Only slices with a
    /// length of 10, 20 or 30 ms are supported, so for example at 8 kHz, `frame.len()`
    /// must be either 80, 160 or 240.
    ///
    /// Returns:             
    /// - Some(true) on active voice detection
    /// - Some(false) on no active voice detection
    /// - None on invalid frame length
    pub fn is_voice_frame(&mut self, frame: &[i16]) -> Option<bool> {
        match unsafe { ffi::fvad_process(self.fvad, frame.as_ptr(), frame.len() as u64) } {
            -1 => None,
            0 => Some(false),
            1 => Some(true),
            n => panic!("fvad_process() returned {}", n),
        }
    }
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

unsafe impl Send for Fvad {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_into_sample_rate() {
        assert_eq!(SampleRate::try_from(7999), Err(()));
        assert_eq!(SampleRate::try_from(8000), Ok(SampleRate::Rate8kHz));
        assert_eq!(SampleRate::try_from(16000), Ok(SampleRate::Rate16kHz));
        assert_eq!(SampleRate::try_from(32000), Ok(SampleRate::Rate32kHz));
        assert_eq!(SampleRate::try_from(48000), Ok(SampleRate::Rate48kHz));
    }

    #[test]
    fn set_mode() {
        Fvad::new()
            .unwrap()
            .set_mode(Mode::Quality)
            .set_mode(Mode::LowBitrate)
            .set_mode(Mode::Aggressive)
            .set_mode(Mode::VeryAggressive);
    }

    #[test]
    fn set_sample_rate() {
        Fvad::new()
            .unwrap()
            .set_sample_rate(SampleRate::Rate8kHz)
            .set_sample_rate(SampleRate::Rate16kHz)
            .set_sample_rate(SampleRate::Rate32kHz)
            .set_sample_rate(SampleRate::Rate48kHz);
    }

    #[test]
    fn is_voice_frame() {
        assert_eq!(
            Fvad::new()
                .unwrap()
                .is_voice_frame(&std::iter::repeat(0).take(160).collect::<Vec<i16>>()),
            Some(false)
        );
    }
}

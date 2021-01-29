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

pub enum Mode {
    Quality,
    LowBitrate,
    Aggresive,
    VeryAggresive,
}

pub enum SampleRate {
    Hz8000,
    Hz16000,
    Hz32000,
    Hz48000,
}

impl TryFrom<u16> for SampleRate {
    type Error = ();

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        match n {
            8000 => Ok(SampleRate::Hz8000),
            16000 => Ok(SampleRate::Hz16000),
            32000 => Ok(SampleRate::Hz32000),
            48000 => Ok(SampleRate::Hz48000),
            _ => Err(()),
        }
    }
}

impl Fvad {
    pub fn new() -> Option<Self> {
        let fvad = unsafe { ffi::fvad_new() };
        match fvad.is_null() {
            false => Some(fvad.into()),
            true => None,
        }
    }

    pub fn reset(&mut self) {
        unsafe { ffi::fvad_reset(self.fvad) }
    }

    pub fn set_mode(self, mode: Mode) -> Option<()> {
        match unsafe {
            ffi::fvad_set_mode(
                self.fvad,
                match mode {
                    Mode::Quality => 0,
                    Mode::LowBitrate => 1,
                    Mode::Aggresive => 2,
                    Mode::VeryAggresive => 3,
                },
            )
        } {
            -1 => None,
            0 => Some(()),
            n => panic!("fvad_set_mode() returned {}", n),
        }
    }

    pub fn set_sample_rate(self, sample_rate: SampleRate) -> Option<()> {
        match unsafe {
            ffi::fvad_set_sample_rate(
                self.fvad,
                match sample_rate {
                    SampleRate::Hz8000 => 8000,
                    SampleRate::Hz16000 => 16000,
                    SampleRate::Hz32000 => 32000,
                    SampleRate::Hz48000 => 48000,
                },
            )
        } {
            -1 => None,
            0 => Some(()),
            n => panic!("fvad_set_sample_rate() returned {}", n),
        }
    }

    pub fn process(self, frame: &[i16]) -> Option<bool> {
        match unsafe { ffi::fvad_process(self.fvad, frame.as_ptr(), frame.len() as u64) } {
            -1 => None,
            0 => Some(false),
            1 => Some(true),
            n => panic!("fvad_process() returned {}", n),
        }
    }
}

use crate::wav::WavMetadata;
use crate::{Frame, FrameWriter, LPCMKind, Sample};
use std::io::{Error, ErrorKind, Result, Write};

pub type WavFrameWriter<W, S> = FrameWriter<W, WavMetadata, S>;

impl<W: Write, S: Sample> WavFrameWriter<W, S> {
    pub fn write_wav_frame(&mut self, wav_frame: Frame<S>) -> Result<()> {
        if wav_frame.0.len() != self.metadata.channels() as usize {
            return Err(ErrorKind::InvalidData.into());
        }

        if self.metadata.frames() <= self.pos {
            return Err(ErrorKind::InvalidData.into());
        } else {
            self.pos += 1;
        }

        for sample in wav_frame.0 {
            sample.write(&mut self.inner)?;
        }

        Ok(())
    }
}

pub enum WavFrameWriterKind<W: Write> {
    F32LE(WavFrameWriter<W, f32>),
    F64LE(WavFrameWriter<W, f64>),
}

impl<W: Write> From<WavFrameWriter<W, f32>> for WavFrameWriterKind<W> {
    fn from(w: WavFrameWriter<W, f32>) -> Self {
        Self::F32LE(w)
    }
}

impl<W: Write> From<WavFrameWriter<W, f64>> for WavFrameWriterKind<W> {
    fn from(w: WavFrameWriter<W, f64>) -> Self {
        Self::F64LE(w)
    }
}

impl<W: Write> WavFrameWriterKind<W> {
    pub fn into_f32_le(self) -> Result<WavFrameWriter<W, f32>> {
        match self {
            Self::F32LE(w) => Ok(w),
            Self::F64LE(w) => Err(Error::new(
                ErrorKind::Other,
                format!(
                    "expected `{:?}`, found `{:?}`",
                    LPCMKind::F32LE,
                    w.metadata.lpcm_kind()
                ),
            )),
        }
    }

    pub fn into_f64_le(self) -> Result<WavFrameWriter<W, f64>> {
        match self {
            Self::F32LE(w) => Err(Error::new(
                ErrorKind::Other,
                format!(
                    "expected `{:?}`, found `{:?}`",
                    LPCMKind::F64LE,
                    w.metadata.lpcm_kind()
                ),
            )),
            Self::F64LE(w) => Ok(w),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write() -> Result<()> {
        let data: Vec<u8> = Vec::new();
        let metadata = WavMetadata {
            frames: 2,
            lpcm_kind: LPCMKind::F32LE,
            channels: 1,
            samples_per_sec: 44100,
        };
        let mut wav_frame_writer = WavFrameWriter::<Vec<u8>, f32>::new(data, metadata);

        wav_frame_writer.write_wav_frame(vec![1.0].into())?;
        wav_frame_writer.write_wav_frame(vec![0.0].into())?;

        assert_eq!(wav_frame_writer.get_ref(), &[0, 0, 0x80, 0x3F, 0, 0, 0, 0]);
        assert!(wav_frame_writer.write_wav_frame(vec![0.0].into()).is_err());
        assert!(wav_frame_writer.write_wav_frame(vec![0.0].into()).is_err());

        Ok(())
    }
}

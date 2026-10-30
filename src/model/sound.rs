use crate::Error;

/// The representation of a Scratch sound inside a sprite.
#[derive(Clone)]
#[non_exhaustive]
pub struct Sound {
    pub name: String,
    pub data_format: SoundFormat,

    /// Raw file data. Enable the `sounds` feature to decode it.
    pub data: Vec<u8>,
}

impl std::fmt::Debug for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            name,
            data_format,
            data,
        } = self;

        f.debug_struct("Costume")
            .field("name", name)
            .field("data_format", data_format)
            .field("data_len", &data.len())
            .finish()
    }
}

/// The sound format the sound is stored in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SoundFormat {
    Wav,
    Mp3,
}

#[cfg(feature = "sounds")]
impl Sound {
    /// Decodes the audio data inside this costume.
    pub fn decode(&self) -> Result<DecodedSound, Error> {
        match self.data_format {
            SoundFormat::Wav => self.decode_wav(),
            SoundFormat::Mp3 => self.decode_mp3(),
        }
    }

    pub fn decode_wav(&self) -> Result<DecodedSound, Error> {
        use hound::{WavReader, WavSpec};

        let wav_reader = WavReader::new(&self.data[..]).map_err(Error::hound)?;
        let WavSpec {
            sample_rate,
            channels,
            ..
        } = wav_reader.spec();
        let samples: Vec<_> = wav_reader
            .into_samples::<i16>()
            .collect::<Result<_, _>>()
            .map_err(Error::hound)?;

        Ok(DecodedSound {
            sample_rate,
            channels,
            samples,
        })
    }

    pub fn decode_mp3(&self) -> Result<DecodedSound, Error> {
        use nanomp3::Reader;
        use std::io::Cursor;

        let cursor = Cursor::new(&self.data[..]);
        let mut mp3_reader: Reader<_, i16> = Reader::new(cursor).map_err(Error::nanomp3)?;
        let sample_rate = mp3_reader.sample_rate();
        let channels = mp3_reader.channels().ok_or(Error::SoundEmpty)?.num() as u16;

        let mut samples = Vec::new();
        let mut buf = [0i16; 4096];

        loop {
            match mp3_reader.read(&mut buf).map_err(Error::nanomp3)? {
                0 => break,
                n => samples.extend_from_slice(&buf[..n]),
            }
        }

        Ok(DecodedSound {
            sample_rate,
            channels,
            samples,
        })
    }
}

/// A decoded sound file.
#[cfg(feature = "sounds")]
pub struct DecodedSound {
    pub sample_rate: u32,
    pub channels: u16,
    pub samples: Vec<i16>,
}

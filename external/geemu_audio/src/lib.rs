use std::collections::HashMap;
use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle};
use std::fs::File;
use std::io::{BufReader, Read, Cursor};
use std::sync::Arc;
use std::error::Error;
use crate::AudioError::SoundNotFound;
use thiserror::Error;

type AudioResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Error)]
enum AudioError {
    #[error("Could not find sound")]
    SoundNotFound,
}

#[derive(Clone)]
struct SoundData {
    data: Arc<Vec<u8>>,
}

impl AsRef<[u8]> for SoundData {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

pub struct Sounds {
    _stream: OutputStream,
    channels: Vec<Sink>,
    sound_store: HashMap<String, SoundData>,
    music_channel: Sink,
}

impl Sounds {
    pub fn new(num_channels: u8) -> AudioResult<Self> {
        let (stream, stream_handle) =  OutputStream::try_default()?;
        Ok(Sounds::init_channels(stream, stream_handle, num_channels))

    }

    fn init_channels(stream: OutputStream, stream_handle: OutputStreamHandle, num_channels: u8) -> Self {
        let mut channels = Vec::new();
        for _i in 0..num_channels {
            let sink = Sink::try_new(&stream_handle).unwrap();
            channels.push(sink);
        }
        let music_channel = Sink::try_new(&stream_handle).unwrap();
        Sounds {
            _stream: stream,
            channels,
            sound_store: HashMap::new(),
            music_channel,
        }
    }

    pub fn play_music(&mut self, file: &str) -> AudioResult<()> {
        let file = BufReader::new(File::open(file)?);
        let source = Decoder::new_looped(file)?;
        Ok(self.music_channel.append(source))
    }

    pub fn load_sound(&mut self, sound: impl Into<String>, file: &str) -> AudioResult<()> {
        let mut file = BufReader::new(File::open(file)?);
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        let data = SoundData {
            data: Arc::new(bytes),
        };
        self.sound_store.insert(sound.into(),  data);
        Ok(())
    }

    pub fn play_sound(&self, sound: impl Into<String>) -> AudioResult<()> {
        let bytes = self.sound_store.get(&sound.into()).ok_or(SoundNotFound)?;
        let cursor = Cursor::new(bytes.clone());
        let source = Decoder::new(cursor)?;
        let free_channel = self.channels.iter().find(|channel| channel.empty());
        if let Some(sink) = free_channel {
            sink.append(source);
        }
        Ok(())
    }

}

use std::collections::HashMap;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{BufReader, Read, Cursor};
use std::hash::Hash;
use std::sync::Arc;

#[derive(Clone)]
struct SoundData {
    data: Arc<Vec<u8>>,
}

impl AsRef<[u8]> for SoundData {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

pub struct Sounds<T> {
    _stream: OutputStream,
    channels: Vec<Sink>,
    sound_store: HashMap<T, SoundData>,
    music_channel: Sink,
}

impl<T: Eq + Hash> Sounds<T> {
    pub fn new(num_channels: u8) -> Self {
        let (stream, stream_handle) =  OutputStream::try_default().unwrap();
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
    pub fn play_music(&mut self, file: &str) {
        let file = BufReader::new(File::open(file).unwrap());
        let source = Decoder::new_looped(file).unwrap();
        self.music_channel.append(source);
    }

    pub fn load_sound(&mut self, sound: T, file: &str) {
        let mut file = BufReader::new(File::open(file).unwrap());
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        let data = SoundData {
          data: Arc::new(bytes),
        };
        self.sound_store.insert(sound,  data);
    }

    pub fn play_sound(&self, sound: &T) {
        let bytes = self.sound_store.get(sound).unwrap();
        let cursor = Cursor::new(bytes.clone());
        let source = Decoder::new(cursor).unwrap();
        let free_channel = self.channels.iter().find(|channel| channel.empty());
        if let Some(sink) = free_channel {
            sink.append(source);
        }
    }
}
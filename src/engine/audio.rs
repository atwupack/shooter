use std::collections::HashMap;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
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
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    channels: Vec<Sink>,
    sound_store: HashMap<T, SoundData>,
}

impl<T: Eq + Hash> Sounds<T> {
    pub fn new(num_channels: u8) -> Self {
        let (stream, stream_handle) =  OutputStream::try_default().unwrap();
        let mut channels = Vec::new();
        for _i in 0..num_channels {
            let sink = Sink::try_new(&stream_handle).unwrap();
            channels.push(sink);
        }
        Sounds {
            stream,
            stream_handle,
            channels,
            sound_store: HashMap::new(),
        }
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

    pub fn play_sound(&self, sound: &T, channel: u8) {
        let bytes = self.sound_store.get(sound).unwrap();
        let cursor = Cursor::new(bytes.clone());
        let source = Decoder::new(cursor).unwrap();
        let sink = self.channels.get(channel as usize).unwrap();
        sink.append(source);
    }
}
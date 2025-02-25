use rodio::OutputStream;
use rodio::OutputStreamHandle;
use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub struct AudioManager {
    pub output_stream: OutputStream,
    pub output_stream_handle: OutputStreamHandle,
}

impl AudioManager {
    pub fn new() -> Self {
        let (output_stream, output_stream_handle) = OutputStream::try_default().unwrap();

        Self {
            output_stream,
            output_stream_handle,
        }
    }

    pub fn play_audio<S>(&self, source: S)
    where
        S: Source<Item = f32> + Send + 'static,
    {
        let play_result = self.output_stream_handle.play_raw(source);

        if play_result.is_err() {
            println!("Failed to play sound: {:?}", play_result.err());
        }
    }
}

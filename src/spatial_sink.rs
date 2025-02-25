use std::f32;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use cpal::FromSample;

use crate::source::Spatial;
use crate::stream::{OutputStreamHandle, PlayError};
use crate::{Sample, Sink, Source};

pub struct SpatialSink {
    sink: Sink,
    positions: Arc<Mutex<SoundPositions>>,
}

struct SoundPositions {
    emitter_position: [f32; 3],
    left_ear: [f32; 3],
    right_ear: [f32; 3],
}

impl SpatialSink {
    /// Builds a new `SpatialSink`.
    pub fn try_new(
        stream: &OutputStreamHandle,
        emitter_position: [f32; 3],
        left_ear: [f32; 3],
        right_ear: [f32; 3],
    ) -> Result<SpatialSink, PlayError> {
        Ok(SpatialSink {
            sink: Sink::try_new(stream)?,
            positions: Arc::new(Mutex::new(SoundPositions {
                emitter_position,
                left_ear,
                right_ear,
            })),
        })
    }

    /// Sets the position of the sound emitter in 3 dimensional space.
    pub fn set_emitter_position(&self, pos: [f32; 3]) {
        self.positions.lock().unwrap().emitter_position = pos;
    }

    /// Sets the position of the left ear in 3 dimensional space.
    pub fn set_left_ear_position(&self, pos: [f32; 3]) {
        self.positions.lock().unwrap().left_ear = pos;
    }

    /// Sets the position of the right ear in 3 dimensional space.
    pub fn set_right_ear_position(&self, pos: [f32; 3]) {
        self.positions.lock().unwrap().right_ear = pos;
    }

    /// Appends a sound to the queue of sounds to play.
    #[inline]
    pub fn append<S>(&self, source: S)
    where
        S: Source + Send + 'static,
        f32: FromSample<S::Item>,
        S::Item: Sample + Send,
    {
        let positions = self.positions.clone();
        let pos_lock = self.positions.lock().unwrap();
        let source = Spatial::new(
            source,
            pos_lock.emitter_position,
            pos_lock.left_ear,
            pos_lock.right_ear,
        )
        .periodic_access(Duration::from_millis(10), move |i| {
            let pos = positions.lock().unwrap();
            i.set_positions(pos.emitter_position, pos.left_ear, pos.right_ear);
        });
        self.sink.append(source);
    }

    // Gets the volume of the sound.
    ///
    /// The value `1.0` is the "normal" volume (unfiltered input). Any value other than 1.0 will
    /// multiply each sample by this value.
    #[inline]
    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    /// Changes the volume of the sound.
    ///
    /// The value `1.0` is the "normal" volume (unfiltered input). Any value other than 1.0 will
    /// multiply each sample by this value.
    #[inline]
    pub fn set_volume(&self, value: f32) {
        self.sink.set_volume(value);
    }

    /// Gets the speed of the sound.
    ///
    /// The value `1.0` is the "normal" speed (unfiltered input). Any value other than `1.0` will
    /// change the play speed of the sound.
    #[inline]
    pub fn speed(&self) -> f32 {
        self.sink.speed()
    }

    /// Changes the speed of the sound.
    ///
    /// The value `1.0` is the "normal" speed (unfiltered input). Any value other than `1.0` will
    /// change the play speed of the sound.
    #[inline]
    pub fn set_speed(&self, value: f32) {
        self.sink.set_speed(value)
    }

    /// Resumes playback of a paused sound.
    ///
    /// No effect if not paused.
    #[inline]
    pub fn play(&self) {
        self.sink.play();
    }

    /// Pauses playback of this sink.
    ///
    /// No effect if already paused.
    ///
    /// A paused sound can be resumed with `play()`.
    pub fn pause(&self) {
        self.sink.pause();
    }

    /// Gets if a sound is paused
    ///
    /// Sounds can be paused and resumed using pause() and play(). This gets if a sound is paused.
    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    /// Removes all currently loaded `Source`s from the `SpatialSink` and pauses it.
    ///
    /// See `pause()` for information about pausing a `Sink`.
    #[inline]
    pub fn clear(&self) {
        self.sink.clear();
    }

    /// Stops the sink by emptying the queue.
    #[inline]
    pub fn stop(&self) {
        self.sink.stop()
    }

    /// Destroys the sink without stopping the sounds that are still playing.
    #[inline]
    pub fn detach(self) {
        self.sink.detach();
    }

    /// Sleeps the current thread until the sound ends.
    #[inline]
    pub fn sleep_until_end(&self) {
        self.sink.sleep_until_end();
    }

    /// Returns true if this sink has no more sounds to play.
    #[inline]
    pub fn empty(&self) -> bool {
        self.sink.empty()
    }

    /// Returns the number of sounds currently in the queue.
    #[inline]
    pub fn len(&self) -> usize {
        self.sink.len()
    }
}

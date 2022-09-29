use std::collections::HashMap;
use thiserror::Error;
use web_sys::HtmlAudioElement;

#[derive(Error, Debug)]
pub enum AudioLibraryError {
    #[error("Couldn't load the resource")]
    FailedToLoad,
    #[error("Couldn't play the resource")]
    FailedToPlay,
    #[error("The resource is missing")]
    Missing,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct AudioLibrary {
    sounds: HashMap<String, HtmlAudioElement>,
}
impl AudioLibrary {
    /// Non-recursively loads all sound files from `path` and creates a new [`AudioLibrary`] from them
    pub fn new_from_dir(path: &str) -> Self {
        todo!();
    }

    /// Tries to load a sound file at `path` and insert it into this [`AudioLibrary`].
    ///
    /// If sound files have the same name, the previous one will be overwritten
    ///
    /// # Errors
    /// This will return [`AudioLibraryError::FailedToLoad`] if the path to file is invalid
    pub fn insert(&mut self, path: &str) -> Result<(), AudioLibraryError> {
        let audio_element = match HtmlAudioElement::new_with_src(path) {
            Ok(this) => this,
            Err(_) => return Err(AudioLibraryError::FailedToLoad),
        };
        self.sounds.insert(path.to_owned(), audio_element);
        Ok(())
    }

    /// Tries to play a sound from this [`AudioLibrary`]
    ///
    /// # Errors
    /// This will return [`AudioLibraryError::Missing`] if sound with this name is not loaded
    pub fn play(&self, name: &str) -> Result<(), AudioLibraryError> {
        match self.sounds.get(name) {
            Some(sound) => {
                if sound.play().is_err() {
                    return Err(AudioLibraryError::FailedToPlay);
                }
            }
            None => return Err(AudioLibraryError::Missing),
        }
        Ok(())
    }
}

use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
#[derive(Debug, Clone, EnumCountMacro, EnumIter)]
pub enum Sound
{
    SoundNone = 0, // no sound
    SoundFocusChange, // played when the focus changes
    SoundFocusError, // played when the user wants to go somewhere impossible (while the highlight wiggles)
    SoundClick, // played when the click action runs
    SoundFocusSidebar, // played when the focus changes to a sidebar item
    SoundClickError, // played when the user clicks a disabled button / a view focused with no click action
    SoundHonk, // honk
    SoundClickSidebar, // played when a sidebar item is clicked
}

// Platform agnostic Audio player
// Each platform's AudioPlayer is responsible for managing the enum Sound -> internal representation map
pub trait AudioPlayer {

    /**
     * Preemptively loads the given sound so that it's ready to be played
     * when needed.
     *
     * Returns a boolean indicating if the sound has been loaded or not.
     */
    fn load(&self, sound: Sound) -> bool;

    /**
     * Plays the given sound.
     *
     * The AudioPlayer should not assume that the sound has been
     * loaded already, and must load it if needed.
     *
     * Returns a boolean indicating if the sound has been played or not.
     */
    fn play(&self, sound: Sound) -> bool;
}

// An AudioPlayer that does nothing
pub struct NullAudioPlayer {

}

impl NullAudioPlayer {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl AudioPlayer for NullAudioPlayer {
    fn load(&self, sound: Sound) -> bool {
        false
    }

    fn play(&self, sound: Sound) -> bool {
        false
    }
}
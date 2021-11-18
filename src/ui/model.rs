use rtrb::Producer;
use vizia::*;

use crate::audio::{self, Controller};

#[derive(Debug, Lens)]
pub struct AppData {
    pub note: wmidi::Note,
    pub control: Producer<audio::engine::Command>,
    pub nodes: Vec<audio::engine::Node>,
}

impl AppData {
    pub fn new() -> Self {
        let Controller {
            active_client: _active_client,
            midi_ui,
            input: audio_input,
        } = audio::start().unwrap();
        AppData {
            note: wmidi::Note::LOWEST_NOTE,
            control: audio_input,
            nodes: Vec::new(),
        }
    }
}

impl Model for AppData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) -> bool {
        if let Some(app_event) = event.message.downcast::<AppEvent>() {
            use AppEvent::*;
            match *app_event {
                NoteIn(note) => {
                    self.note = note;
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AppEvent {
    NoteIn(wmidi::Note),
}

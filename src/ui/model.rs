use std::ops::{Deref, DerefMut};

use rtrb::Producer;
use vizia::*;

use crate::audio::{self, Controller};

#[derive(Debug, Lens)]
pub struct MainModel {
    pub note: Note,
    pub control: Producer<audio::engine::Command>,
    pub nodes: Vec<audio::engine::Node>,
}

impl MainModel {
    pub fn new() -> Self {
        let Controller {
            active_client: _active_client,
            midi_ui,
            input: audio_input,
        } = audio::start().unwrap();
        MainModel {
            note: Note(wmidi::Note::LOWEST_NOTE),
            control: audio_input,
            nodes: Vec::new(),
        }
    }
}

impl Model for MainModel {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast::<AppEvent>() {
            use AppEvent::*;
            match *app_event {
                NoteIn(note) => {
                    self.note.0 = note;
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AppEvent {
    NoteIn(wmidi::Note),
}

#[derive(Clone, Copy, Debug)]
pub struct Note(wmidi::Note);

impl Data for Note {
    fn same(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Deref for Note {
    type Target = wmidi::Note;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Note {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

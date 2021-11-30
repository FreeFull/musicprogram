use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use vizia::*;
use wmidi::MidiMessage;

use crate::audio;

type AudioTx = Rc<RefCell<rtrb::Producer<audio::Command>>>;

#[derive(Debug, Lens)]
pub struct MainModel {
    pub note: Note,
    pub audio_event_tx: AudioTx,
    pub nodes: Vec<audio::Node>,
}

impl MainModel {
    pub fn new(audio_event_tx: AudioTx) -> Self {
        MainModel {
            note: Note(wmidi::Note::LOWEST_NOTE),
            audio_event_tx,
            nodes: Vec::new(),
        }
    }
}

impl Model for MainModel {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        let _ = cx;
        if let Some(app_event) = event.message.downcast::<AppEvent>() {
            use AppEvent::*;
            match *app_event {
                AddNode(kind) => {
                    self.nodes.push(audio::Node::new(kind));
                }
                RemoveNode(index) => {
                    self.nodes.remove(index);
                }
                MidiIn(ref midi_message) => match *midi_message {
                    MidiMessage::NoteOn(_channel, note, _velocity) => {
                        self.note.0 = note;
                    }
                    _ => {}
                },
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    AddNode(audio::NodeKind),
    RemoveNode(usize),
    MidiIn(wmidi::MidiMessage<'static>),
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

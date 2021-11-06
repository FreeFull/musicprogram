use rtrb::Producer;
use tuix::*;

use crate::audio;

#[derive(Debug, Lens)]
pub struct AppData {
    pub note: wmidi::Note,
    pub control: Producer<audio::engine::Command>,
    pub nodes: NodeData,
}

impl AppData {
    pub fn new(control: Producer<audio::engine::Command>) -> Self {
        AppData {
            note: wmidi::Note::LOWEST_NOTE,
            control,
            nodes: NodeData { nodes: Vec::new() },
        }
    }
}

impl Model for AppData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast::<AppEvent>() {
            use AppEvent::*;
            match *app_event {
                NoteIn(note) => {
                    self.note = note;
                    entity.update(state);
                }
            }
        }
    }
}

pub enum AppEvent {
    NoteIn(wmidi::Note),
}

#[derive(Debug)]
pub struct NodeData {
    pub nodes: Vec<audio::engine::Node>,
}

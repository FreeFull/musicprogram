use rtrb::Producer;
use tuix::*;

use crate::audio;

#[derive(Debug)]
pub struct AppData {
    pub note: wmidi::Note,
    pub control: Producer<audio::engine::Command>,
}

impl Model for AppData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast::<AppEvent>() {
            dbg!("AppEvent received");

            use AppEvent::*;
            match *app_event {
                NoteIn(note) => {
                    self.note = note;
                    entity.update(state);
                }
            }
        }
    }

    fn build(self, state: &mut State, parent: Entity) -> Entity
    where
        Self: std::marker::Sized + Model + Node,
    {
        Store::new(self).build(state, parent, |builder| builder)
    }
}

pub enum AppEvent {
    NoteIn(wmidi::Note),
}

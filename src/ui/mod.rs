use std::cell::RefCell;

use crate::audio::Controller;

use super::audio;

use tuix::*;

mod model;

pub use model::*;

pub fn start() {
    let Controller {
        active_client: _active_client,
        midi_ui,
        input: audio_input,
    } = audio::start().unwrap();
    let midi_ui = RefCell::new(midi_ui);
    let window_desc = WindowDescription::new().with_title("musicprogram");
    let app = Application::new(window_desc, |state, window| {
        let app_data = AppData {
            control: audio_input,
            note: wmidi::Note::A0,
        }
        .build(state, window);
    })
    .on_idle(move |state| {
        if let Ok(message) = midi_ui.borrow_mut().pop() {
            match message {
                wmidi::MidiMessage::NoteOn(_, note, _) => {
                    state.insert_event(
                        Event::new(AppEvent::NoteIn(note)).propagate(Propagation::All),
                    );
                }
                _ => {}
            }
        }
    });
    app.run();
}

use std::cell::RefCell;

use crate::audio::Controller;

use super::audio;

use tuix::widgets::*;
use tuix::*;

mod model;
mod node_list;

pub use model::*;
use node_list::*;

pub fn start() {
    let Controller {
        active_client: _active_client,
        midi_ui,
        input: audio_input,
    } = audio::start().unwrap();
    let midi_ui = RefCell::new(midi_ui);
    let window_desc = WindowDescription::new().with_title("musicprogram");
    let app = Application::new(window_desc, |state, window| {
        state.add_stylesheet("style.css").ok();
        let app_data = AppData::new(audio_input).build(state, window);
        let status_bar = Row::new().build(state, app_data, |builder| builder.class("status-bar"));
        Label::new("")
            .bind(AppData::note, |note| note.to_string())
            .build(state, status_bar, |builder| builder.class("current-note"));
    })
    .should_poll()
    .on_idle(move |state| {
        while let Ok(message) = midi_ui.borrow_mut().pop() {
            match message {
                wmidi::MidiMessage::NoteOn(_, note, _) => {
                    dbg!(note);
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

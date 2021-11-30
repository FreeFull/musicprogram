use std::{cell::RefCell, rc::Rc};

use vizia::*;

mod model;
mod views;

use crate::audio;
pub use model::*;

pub fn start() {
    let window_desc = WindowDescription::new().with_title("musicprogram");
    let mut controller = audio::start().unwrap();
    let audio_tx = Rc::new(RefCell::new(controller.input));
    let app = Application::new(window_desc, move |cx| {
        cx.add_stylesheet("style.css").ok();
        model::MainModel::new(audio_tx.clone()).build(cx);
        ZStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                views::node_list::build(cx);
                HStack::new(cx, |cx| {
                    Binding::new(cx, model::MainModel::note, |cx, note| {
                        Label::new(cx, note.get(cx).to_str()).class("current-note");
                    });
                })
                .class("status-bar");
            });
            views::modals::build(cx);
        });
    });
    let proxy = app.get_proxy();
    std::thread::spawn(move || loop {
        while let Ok(midi_message) = controller.midi_ui.pop() {
            if let Err(_) = proxy.send_event(Event::new(AppEvent::MidiIn(midi_message))) {
                return;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    });
    app.run();
}

impl Data for crate::audio::Node {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

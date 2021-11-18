use std::{cell::RefCell, rc::Rc};

use crate::audio::Controller;

use super::audio;

use vizia::{views::*, *};

mod model;
mod node_list;

pub use model::*;

pub fn start() {
    let data = Rc::new(RefCell::new(()));
    Application::new(move |cx| {
        cx.add_stylesheet("style.css").ok();
        AppData::new().build(cx);
        node_list::build(cx);
        let status_bar = HStack::new(cx, |cx| {
            Binding::new(cx, AppData::note, |cx, note| {
                Label::new(cx, note.get(cx).to_str()).class("current-note");
            });
        })
        .class("status-bar");
    })
    .run();
}

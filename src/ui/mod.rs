use std::{cell::RefCell, rc::Rc};

use crate::audio::Controller;

use vizia::*;

mod model;
mod views;

pub use model::*;

pub fn start() {
    let data = Rc::new(RefCell::new(()));
    Application::new(move |cx| {
        cx.add_stylesheet("style.css").ok();
        model::MainModel::new().build(cx);
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
    })
    .run();
}

impl Data for crate::audio::Node {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

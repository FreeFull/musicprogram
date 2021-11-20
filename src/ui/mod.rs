use std::{cell::RefCell, rc::Rc};

use crate::audio::Controller;

use super::audio;

use vizia::*;

mod modals;
mod model;
mod node_list;

pub use model::*;

pub fn start() {
    let data = Rc::new(RefCell::new(()));
    Application::new(move |cx| {
        cx.add_stylesheet("style.css").ok();
        model::MainModel::new().build(cx);
        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                node_list::build(cx);
                HStack::new(cx, |cx| {
                    Binding::new(cx, model::MainModel::note, |cx, note| {
                        Label::new(cx, note.get(cx).to_str()).class("current-note");
                    });
                })
                .class("status-bar");
            })
            .position_type(PositionType::SelfDirected);
            modals::build(cx);
        });
    })
    .run();
}

impl Data for crate::audio::engine::Node {
    fn same(&self, other: &Self) -> bool {
        todo!()
    }
}

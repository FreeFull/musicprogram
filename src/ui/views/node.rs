use vizia::*;

use crate::{
    audio,
    ui::{AppEvent, MainModel},
};

pub struct Node {
    data: audio::Node,
    index: usize,
}

impl Node {
    pub fn new(cx: &mut Context, data: audio::Node, index: usize) -> Handle<Self> {
        Node { data, index }.build(cx)
    }
}

impl View for Node {
    fn body(&mut self, cx: &mut Context) {
        let index = self.index;
        let mut node = self.data;
        HStack::new(cx, move |cx| {
            for input in node.inputs() {
                Label::new(cx, input.name).class("input");
                Knob::new(cx, 0.0, StaticLens::new(&0.5), true).on_changing(move |knob, cx| {
                    println!("{}", knob.current);
                });
            }
            for output in node.outputs() {
                Label::new(cx, output.name).class("output");
            }
            Button::new(
                cx,
                move |cx| {
                    cx.emit(AppEvent::RemoveNode(index));
                },
                |cx| {
                    Label::new(cx, "X")
                        .child_space(Stretch(1.0))
                        .width(Stretch(1.0))
                        .height(Stretch(1.0))
                },
            )
            .class("delete");
        })
        .class("node");
    }
}

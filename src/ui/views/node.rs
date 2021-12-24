use vizia::*;

use crate::ui::{AppEvent, MainModel};

pub struct Node {
    index: usize,
}
impl Node {
    pub fn new(cx: &mut Context, index: usize) -> Handle<Self> {
        Node { index }.build(cx)
    }
}

impl View for Node {
    fn body(&mut self, cx: &mut Context) {
        let index = self.index;
        Binding::new(cx, MainModel::nodes, move |cx, nodes| {
            HStack::new(cx, move |cx| {
                let mut node = nodes.get(cx)[index].clone();
                for input in node.inputs() {
                    Label::new(cx, input.name).class("input");
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
        });
    }
}

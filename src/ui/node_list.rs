use vizia::*;

use super::{modals::ModalEvent, model::*};
use crate::audio::Node;

pub fn build(cx: &mut Context) {
    HStack::new(cx, |cx| {
        ListData {
            selected: 0,
            length: 0,
        }
        .build(cx);
        List::new(cx, MainModel::nodes, node).class("list");
        VStack::new(cx, |cx| {
            Button::new(
                cx,
                |cx| {
                    println!("Add Node pressed");
                    cx.emit(ModalEvent::Show("add node"));
                },
                |cx| {
                    Label::new(cx, "Add");
                },
            );
            Button::new(
                cx,
                |cx| {},
                |cx| {
                    Label::new(cx, "Remove");
                },
            );
        });
    })
    .width(Stretch(1.0))
    .height(Stretch(1.0));
}

fn node(cx: &mut Context, ptr: ItemPtr<impl Lens<Target = Vec<Node>>, Node>) {
    Label::new(cx, ptr.value(cx).name());
}

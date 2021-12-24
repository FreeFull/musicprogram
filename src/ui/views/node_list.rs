use vizia::*;

use crate::ui::{model::*, views};

pub fn build(cx: &mut Context) {
    HStack::new(cx, |cx| {
        List::new(cx, MainModel::nodes, |cx, node| {
            HStack::new(cx, move |cx| {
                Label::new(cx, node.value(cx).name());
                views::Node::new(cx, node.index());
            })
            .height(Auto);
        })
        .class("list");
        VStack::new(cx, |cx| {
            Button::new(
                cx,
                |cx| {
                    cx.emit(views::ModalEvent::Show("add node"));
                },
                |cx| Label::new(cx, "Add"),
            );
        })
        .width(Auto);
    })
    .width(Stretch(1.0))
    .height(Stretch(1.0));
}

use vizia::{views::*, *};

use super::model::*;
use crate::audio::engine::Node;

pub fn build(cx: &mut Context) {
    HStack::new(cx, |cx| {
        List::new(cx, AppData::nodes, node).class("list");
        VStack::new(cx, |cx| {
            Button::new(
                cx,
                |cx| {},
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

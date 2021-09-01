use cursive::direction::Orientation::*;
use cursive::traits::Nameable;
use cursive::views::SelectView;
use cursive::{views, Cursive, View};

use crate::audio::engine;
use crate::audio::engine::{Command, Node, NodeKind};
use crate::ui::UiData;

pub fn build() -> impl View {
    let stack_list = views::SelectView::<Node>::new().with_name("stack");
    let add = views::Button::new("Add new", add_node_dialog);
    let edit = views::Button::new("Edit", edit_node_dialog);
    views::Panel::new(
        views::LinearLayout::new(Horizontal)
            .child(stack_list)
            .child(views::LinearLayout::new(Vertical).child(add).child(edit)),
    )
}

fn push(siv: &mut Cursive, node: engine::Node) {
    siv.call_on_name("stack", |view: &mut SelectView<Node>| {
        view.add_item(node.name(), node);
    });
    update(siv);
}

fn update(siv: &mut Cursive) {
    if let Some(mut data) = siv.take_user_data::<UiData>() {
        let nodes = siv
            .call_on_name("stack", |stack: &mut SelectView<Node>| {
                stack.iter().map(|x| x.1.clone()).collect::<Vec<_>>()
            })
            .unwrap();
        if let Err(_) = data.engine.input.push(Command::ReplaceNodes(nodes)) {
            siv.add_layer(
                views::Dialog::new()
                    .content(views::TextView::new("content"))
                    .dismiss_button("Close"),
            )
        }
        siv.set_user_data(data);
    }
}

fn add_node_dialog(siv: &mut Cursive) {
    let mut list = cursive::views::SelectView::<NodeKind>::new();
    for node_kind in NodeKind::iter() {
        list.add_item(node_kind.name(), node_kind);
    }
    list.set_on_submit(|siv, &node_kind: &NodeKind| {
        let node = Node::new(node_kind);
        push(siv, node);
        siv.pop_layer();
    });
    let view = cursive::views::Dialog::new()
        .title("Add Node")
        .content(list)
        .dismiss_button("Close");
    siv.add_layer(view);
}

fn edit_node_dialog(siv: &mut Cursive) {
    todo!()
}

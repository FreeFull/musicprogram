use std::cell::RefCell;
use std::rc::{Rc, Weak};

use cursive::traits::{Boxable, Nameable, Scrollable};
use cursive::view::ViewWrapper;
use cursive::views::LinearLayout;
use cursive::{views, Cursive, View};

use super::spinner::*;
use crate::audio::engine::{self, PortKind};
use crate::audio::engine::{Command, Node, NodeKind};
use crate::ui::UiData;

pub fn build() -> impl View {
    let stack_list = views::LinearLayout::vertical()
        .with_name("stack")
        .scrollable()
        .full_width();
    let add = views::Button::new("  Add new", add_node_dialog);
    let edit = views::Button::new("     Edit", edit_node_dialog);
    let move_up = views::Button::new("  Move Up", |siv| {
        siv.call_on_name("stack", |stack: &mut LinearLayout| {
            let current = stack.get_focus_index();
            if current == 0 {
                return;
            }
            stack.swap_children(current, current - 1);
        });
    });
    let move_down = views::Button::new("Move Down", |siv| {
        siv.call_on_name("stack", |stack: &mut LinearLayout| {
            let current = stack.get_focus_index();
            if current == stack.len() - 1 {
                return;
            }
            stack.swap_children(current, current + 1);
        });
    });
    views::Panel::new(
        views::LinearLayout::horizontal().child(stack_list).child(
            views::LinearLayout::vertical()
                .child(add)
                .child(edit)
                .child(move_up)
                .child(move_down),
        ),
    )
}

fn push(siv: &mut Cursive, node: engine::Node) {
    siv.call_on_name("stack", |view: &mut LinearLayout| {
        view.add_child(NodeView::new(node));
    });
    update(siv);
}

fn update(siv: &mut Cursive) {
    if let Some(mut data) = siv.take_user_data::<UiData>() {
        let nodes = siv
            .call_on_name("stack", |stack: &mut LinearLayout| {
                (0..stack.len())
                    .map(|i| {
                        stack
                            .get_child(i)
                            .and_then(|view| view.downcast_ref::<NodeView>())
                            .map(|view| view.node.borrow().clone())
                    })
                    .flatten()
                    .collect()
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
    let mut list = views::SelectView::<NodeKind>::new();
    for node_kind in NodeKind::iter() {
        list.add_item(node_kind.name(), node_kind);
    }
    list.set_on_submit(|siv, &node_kind: &NodeKind| {
        let node = Node::new(node_kind);
        push(siv, node);
        siv.pop_layer();
    });
    let view = views::Dialog::new()
        .title("Add Node")
        .content(list)
        .dismiss_button("Close");
    siv.add_layer(view);
}

fn edit_node_dialog(siv: &mut Cursive) {
    let view = views::TextView::new("todo");
    let dialog = views::Dialog::new()
        .content(view)
        .button("Apply", |siv| {
            update(siv);
            todo!();
        })
        .dismiss_button("Cancel");
    siv.add_layer(dialog);
}

struct NodeView {
    view: LinearLayout,
    node: Rc<RefCell<Node>>,
}

impl NodeView {
    fn new(node: Node) -> NodeView {
        let name = node.name();
        let node = Rc::new(RefCell::new(node));
        let mut input_ports = LinearLayout::horizontal();
        let mut output_ports = LinearLayout::horizontal();
        let view = LinearLayout::horizontal().child(views::TextView::new(name));

        for i in 0..node.borrow_mut().inputs().count() {
            input_ports.add_child(InputPortView::new(Rc::downgrade(&node), i));
        }

        for i in 0..node.borrow_mut().outputs().count() {
            output_ports.add_child(OutputPortView::new(Rc::downgrade(&node), i));
        }

        NodeView {
            view: view.child(input_ports).child(output_ports),
            node,
        }
    }
}

impl ViewWrapper for NodeView {
    cursive::wrap_impl!(self.view: LinearLayout);
}

struct InputPortView {
    view: LinearLayout,
}

impl ViewWrapper for InputPortView {
    cursive::wrap_impl!(self.view: LinearLayout);
}

impl InputPortView {
    fn new(_node: Weak<RefCell<Node>>, _port: usize) -> Self {
        InputPortView {
            view: LinearLayout::vertical().child(views::TextView::new("todo")),
        }
    }
}

struct OutputPortView {
    view: LinearLayout,
}

impl ViewWrapper for OutputPortView {
    cursive::wrap_impl!(self.view: LinearLayout);
}

impl OutputPortView {
    fn new(node: Weak<RefCell<Node>>, port: usize) -> Self {
        let mut view = LinearLayout::vertical();
        let mut toggle: views::RadioGroup<PortKind> = views::RadioGroup::new();
        view.add_child(toggle.button(PortKind::Audio([0.0; 256]), "Audio"));
        view.add_child(toggle.button(PortKind::Control(0.0), "Control"));
        toggle.set_on_change(move |_siv, &port_kind| {
            node.upgrade().map(|rc| {
                rc.borrow_mut().outputs().nth(port).map(|port| {
                    port.kind = port_kind;
                })
            });
        });
        view.add_child(Spinner::new(0, 100, 0));
        OutputPortView { view }
    }
}

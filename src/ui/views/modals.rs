use crate::audio;
use vizia::*;

use crate::ui::AppEvent;

pub fn build(cx: &mut Context) {
    ModalManager::default().build(cx);
    modal(
        cx,
        "add node",
        |cx| {
            VStack::new(cx, |cx| {
                for node in audio::NodeKind::iter() {
                    Button::new(
                        cx,
                        move |cx| {
                            cx.emit(AppEvent::AddNode(node));
                            cx.emit(ModalEvent::Hide);
                        },
                        move |cx| {
                            Label::new(cx, node.name());
                        },
                    );
                }
            });
        },
        |cx| {
            Button::new(
                cx,
                |cx| {
                    cx.emit(ModalEvent::Hide);
                },
                |cx| {
                    Label::new(cx, "Close");
                },
            );
        },
    )
}

fn modal(
    cx: &mut Context,
    id: &'static str,
    content: impl Fn(&mut Context) + Copy + 'static,
    buttons: impl Fn(&mut Context) + Copy + 'static,
) {
    Binding::new(cx, ModalManager::visible, move |cx, field| {
        VStack::new(cx, move |cx| {
            content(cx);
            HStack::new(cx, buttons).class("buttons");
        })
        .class("modal")
        .display(if field.get(cx) == &Some(id) {
            Display::Flex
        } else {
            Display::None
        });
    });
}

#[derive(Default, Lens)]
pub struct ModalManager {
    visible: Option<&'static str>,
}

impl Model for ModalManager {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        let _ = cx;
        if let Some(event) = event.message.downcast() {
            dbg!(*event);
            use ModalEvent::*;
            match *event {
                Show(id) => {
                    self.visible = Some(id);
                }
                Hide => {
                    self.visible = None;
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ModalEvent {
    Show(&'static str),
    Hide,
}

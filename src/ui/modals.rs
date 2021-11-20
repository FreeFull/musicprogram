use vizia::*;

pub fn build(cx: &mut Context) {
    EnabledModal::default().build(cx);
    modal(
        cx,
        "add node",
        |cx| {
            struct Data {}
            impl Model for Data {}
            Data {}.build(cx);
            Label::new(cx, "New Node modal");
        },
        |cx| {
            Button::new(
                cx,
                |cx| {
                    println!("Close button");
                }, //cx.emit(ModalEvent::Hide),
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
    Binding::new(cx, EnabledModal::visible, move |cx, field| {
        let entity = VStack::new(cx, move |cx| {
            content(cx);
            HStack::new(cx, buttons).class("buttons");
        })
        .class("modal")
        .entity;
        let visible = field.get(cx) == &Some(id);
        cx.style.borrow_mut().display.insert(
            entity,
            if visible {
                Display::Flex
            } else {
                Display::None
            },
        );
    })
    .position_type(PositionType::SelfDirected);
}

#[derive(Default, Lens)]
pub struct EnabledModal {
    visible: Option<&'static str>,
}

impl Model for EnabledModal {
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

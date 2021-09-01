use super::audio;
use cursive::{
    direction::Orientation::{Horizontal, Vertical},
    menu::MenuTree,
    traits::{Boxable, Nameable},
    views,
    views::TextView,
    View,
};

mod stack;

struct UiData {
    note: wmidi::Note,
    engine: audio::Controller,
}

pub fn start() {
    let mut siv = cursive::default().into_runner();
    siv.set_autorefresh(true);
    siv.set_user_data(UiData {
        engine: audio::start().unwrap(),
        note: wmidi::Note::A0,
    });
    let main = build();
    siv.add_global_callback('q', |s| s.quit());
    siv.add_fullscreen_layer(main);
    siv.set_autohide_menu(false);
    siv.menubar().add_subtree(
        "File",
        MenuTree::new()
            .leaf("New", |_siv| todo!())
            .leaf("Open", |_siv| todo!())
            .leaf("Save", |_siv| todo!())
            .leaf("Save as", |_siv| todo!())
            .delimiter()
            .leaf("Quit", |siv| siv.quit()),
    );
    while siv.is_running() {
        siv.with_user_data(|data: &mut UiData| {
            while let Ok(message) = data.engine.midi_ui.pop() {
                match message {
                    wmidi::MidiMessage::NoteOn(_, _note, _) => {
                        data.note = _note;
                    }
                    _ => {}
                }
            }
        });
        let note = siv.user_data::<UiData>().unwrap().note;
        siv.call_on_name("note", move |n: &mut TextView| n.set_content(note.to_str()));
        siv.step();
    }
}

pub fn build() -> impl View {
    let mut layout = views::LinearLayout::new(Vertical);
    layout.add_child(stack::build());
    let status_bar = views::LinearLayout::new(Horizontal)
        .child(views::TextView::new("").center().with_name("note"))
        .full_width();
    layout.add_child(status_bar);
    views::CircularFocus::new(layout, true, true)
}

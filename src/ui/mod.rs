use super::audio::{self, engine::NodeKind};
use cursive::traits::*;
use cursive::views::TextView;
use cursive::Cursive;

pub fn start() {
    let mut engine = audio::start().unwrap();
    let mut siv = cursive::default().into_runner();
    build(&mut *siv);
    let mut note = wmidi::Note::A0;
    while siv.is_running() {
        while let Ok(data) = engine.midi_ui.pop() {
            match data {
                wmidi::MidiMessage::NoteOn(_, _note, _) => {
                    note = _note;
                }
                _ => {}
            }
        }
        siv.call_on_name("note", move |n: &mut TextView| n.set_content(note.to_str()));
        siv.step();
    }
}

fn build(siv: &mut Cursive) {
    let display = TextView::new("Test").center().with_name("note");
    siv.set_autorefresh(true);
    siv.add_layer(display);
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('n', add_node_dialog);
}

fn add_node_dialog(siv: &mut Cursive) {
    let mut list = cursive::views::SelectView::<NodeKind>::new();
    for node_kind in NodeKind::iter() {
        list.add_item(node_kind.name(), node_kind);
    }
    let view = cursive::views::Dialog::new()
        .title("Add Node")
        .content(list)
        .dismiss_button("Cancel");
    siv.add_layer(view);
}

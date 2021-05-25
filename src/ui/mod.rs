use cursive::traits::*;
use cursive::views::TextView;
use super::engine;

pub fn start() {
    let mut siv = cursive::default().into_runner();
    let mut engine = engine::start().unwrap();
    let display = TextView::new("Test").center().with_name("note");
    siv.set_autorefresh(true);
    siv.add_layer(display);
    siv.add_global_callback('q', |s| s.quit());
    let mut note = wmidi::Note::A0;
    while siv.is_running() {
        while let Ok(data) = engine.data_in.pop() {
            note = data;
        }
        siv.call_on_name("note", move |n: &mut TextView| n.set_content(note.to_str()));
        siv.step();
    }
}
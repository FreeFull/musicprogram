use cursive::{
    direction::Direction,
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    traits::*,
    Printer, Vec2,
};

#[derive(Debug, Copy, Clone)]
pub struct Spinner {
    min: i64,
    max: i64,
    value: i64,
    focus: SpinnerFocus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SpinnerFocus {
    Number,
    Decrement,
    Increment,
}

impl Spinner {
    pub fn new(min: i64, max: i64, default: i64) -> Self {
        Spinner {
            min,
            max,
            value: default,
            focus: SpinnerFocus::Number,
        }
    }

    pub fn decrement(&mut self) {
        if self.value > self.min {
            self.value = self.value - 1;
        }
    }

    pub fn increment(&mut self) {
        if self.value < self.max {
            self.value = self.value + 1;
        }
    }

    pub fn get(&self) -> i64 {
        self.value
    }
}

impl View for Spinner {
    fn draw(&self, printer: &Printer) {
        let number = format!("{:<1$}", self.value, printer.size.x - 2);
        printer
            .focused(self.focus == SpinnerFocus::Number)
            .print((0, 0), &number);
        printer
            .focused(self.focus == SpinnerFocus::Decrement)
            .print((printer.size.x - 2, 0), "↓");
        printer
            .focused(self.focus == SpinnerFocus::Increment)
            .print((printer.size.x - 1, 0), "↑");
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(
            std::cmp::max(number_length(self.min), number_length(self.max)) + 2,
            1,
        )
    }

    fn take_focus(&mut self, _source: Direction) -> bool {
        true
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char(x @ '0'..='9') => {
                self.value *= 10;
                self.value += x as i64 - '0' as i64;
                self.value = self.value.clamp(self.min, self.max);
                EventResult::Consumed(None)
            }
            Event::Char('-') => {
                self.value = self.value.saturating_neg();
                self.value = self.value.clamp(self.min, self.max);
                EventResult::Consumed(None)
            }
            Event::Key(Key::Tab) => match self.focus {
                SpinnerFocus::Number => {
                    self.focus = SpinnerFocus::Decrement;
                    EventResult::Consumed(None)
                }
                SpinnerFocus::Decrement => {
                    self.focus = SpinnerFocus::Increment;
                    EventResult::Consumed(None)
                }
                SpinnerFocus::Increment => EventResult::Ignored,
            },
            Event::Key(Key::Backspace) => {
                self.value /= 10;
                EventResult::Consumed(None)
            }
            Event::Shift(Key::Tab) => match self.focus {
                SpinnerFocus::Number => EventResult::Ignored,
                SpinnerFocus::Decrement => {
                    self.focus = SpinnerFocus::Number;
                    EventResult::Consumed(None)
                }
                SpinnerFocus::Increment => {
                    self.focus = SpinnerFocus::Decrement;
                    EventResult::Consumed(None)
                }
            },
            Event::Mouse {
                offset,
                position,
                event,
            } => match event {
                MouseEvent::Press(MouseButton::Left) => {
                    if let Some(_position) = position.checked_sub(offset) {}
                    todo!()
                }
                MouseEvent::WheelUp => {
                    self.increment();
                    EventResult::Consumed(None)
                }
                MouseEvent::WheelDown => {
                    self.decrement();
                    EventResult::Consumed(None)
                }
                _ => EventResult::Ignored,
            },
            _ => EventResult::Ignored,
        }
    }
}

fn number_length(mut x: i64) -> usize {
    if x == 0 {
        return 1;
    }
    let mut len = 0;
    if x < 0 {
        len += 1;
        x = -x;
    }
    while x > 0 {
        x /= 10;
        len += 1;
    }
    len
}

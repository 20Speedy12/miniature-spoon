
use iced::widget::{button, column, text};
use iced::Element;

fn main() -> iced::Result {
    iced::run("Counter", update, view)
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i32,
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
        Message::Decrement => counter.value -= 1,
    }
}

fn view(counter: &Counter) -> Element<'_, Message> {
    column![
        text(counter.value).size(40),
        button("+").on_press(Message::Increment),
        button("-").on_press(Message::Decrement),
    ]
    .spacing(10)
    .into()
}

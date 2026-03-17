use iced::Element;
use iced::Length::Fill;
use iced::widget::column;
use iced::widget::container;
use iced::widget::scrollable;
use iced::widget::space;
use iced::widget::button;
use iced::widget::text;

#[derive(Debug, Clone)]
pub enum Message {
  ShrinkOn,
  ShrinkOff,
  ShrinkToggle,
  AddTab(String),
  RemoveTab(String),
}

#[derive(Clone, Debug)]
pub struct SideBar {
  tabs: Vec<String>,
  is_shrink: bool,
}

impl SideBar {
  pub fn new() -> Self {
    Self {
      tabs: vec!["greetings".to_owned()],
      is_shrink: false,
    }
  }

  pub fn view<'a>(self: &'a Self) -> iced::Element<'a, Message> {
    let tabs: Vec<Element<Message>> = if self.is_shrink {
      self
        .tabs
        .iter()
        .enumerate()
        .map(|(i, _)| text(i).into())
        .collect()
    } else {
      self.tabs.iter().map(|i| text(i).into()).collect()
    };

    let bar = container(column![
      scrollable(column(tabs).padding(10)),
      space().height(Fill),
      button(if self.is_shrink { ">"} else { "shrink" })
      .width(Fill)
      .style(button::subtle)
      .on_press(Message::ShrinkToggle)
    ])
    .padding(10)
    .height(Fill)
    .width(if self.is_shrink { 50 } else { 100 })
    .max_width(100)
    .style(container::bordered_box);

    container(bar).padding(6).into()
  }

  pub fn update(&mut self, msg: Message) {
    match msg {
      Message::ShrinkToggle => {
        self.is_shrink = !self.is_shrink;
      }

      Message::ShrinkOn => {
        self.is_shrink = true;
      }

      Message::ShrinkOff => {
        self.is_shrink = false;
      }

      Message::AddTab(s) => {
        self.tabs.push(s);
      }

      Message::RemoveTab(s) => {}
    }
  }
}

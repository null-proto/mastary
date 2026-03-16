use iced::Length::Fill;
use iced::widget::container;
use iced::widget::row;
use iced::widget::text;

#[derive(Debug, Clone)]
pub enum Message {
  ShrinkOn,
  ShrinkOff,
  AddTab(String),
  RemoveTab(String),
}

#[derive(Clone, Debug)]
#[allow(unused)]
pub struct SideBar {
  tabs: Vec<String>,
  is_shrink: bool,
}

impl SideBar {
  pub fn new() -> Self {
    Self {
      tabs: vec![],
      is_shrink: false,
    }
  }

  pub fn view<'a>(self: &'a Self) -> iced::Element<'a, Message> {
    let bar = container(row![
      text!("greetings"),
      // space().height(Fill),
      text!("new window"),
    ])
    .padding(10)
    .height(Fill)
    .width(100)
    .max_width(100)
    .style(container::bordered_box);

    container(bar).padding(10).into()
  }

  pub fn update(&mut self, msg: Message) {

  }
}

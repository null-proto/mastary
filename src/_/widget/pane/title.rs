use iced::Element;
use iced::widget::container;
use iced::widget::pane_grid::TitleBar;
use iced::widget::row;
use iced::widget::space;
use iced::widget::text;
use iced::Alignment;

pub struct Title;

use crate::ui::UpdateMessage;

impl Title {
  pub fn new() -> Self {
    Self
  }

  pub fn view(&self) -> TitleBar<'_, UpdateMessage> {
    let title = container(text!("title"))
      .align_y(Alignment::Center)
      .padding(10);

    TitleBar::new(title)
  }
}

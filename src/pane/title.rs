use iced::widget::button;
use iced::widget::container;
use iced::widget::pane_grid::Pane;
use iced::widget::pane_grid::TitleBar;
use iced::widget::pane_grid::Controls;
use iced::widget::row;
use iced::widget::text;

use super::Message;

pub fn view<'a>(
  pane: Pane,
) -> TitleBar<'a, Message> {
  let content = row![
    button(text("Close")).on_press(Message::PaneClose(pane)),
  ];

  let f = format!("ID: {:?}", pane );

  let title = container(text(f)).padding([0,4]);


  TitleBar::new(title)
    .controls(Controls::new(content))
}

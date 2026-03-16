use iced::Alignment;
use iced::Length;
use iced::widget::button;
use iced::widget::container;
use iced::widget::pane_grid::Pane;
use iced::widget::pane_grid::TitleBar;
use iced::widget::pane_grid::Controls;
use iced::widget::responsive;
use iced::widget::row;
use iced::widget::text;
use iced::widget::Space;

use super::Message;
use super::Panels;
use crate::Mastary;

pub fn view<'a>(
  state: &'a Mastary,
  pane: Pane,
  panel: &Panels,
  maximized: bool,
) -> TitleBar<'a, super::Message> {
  let content = row![
    button(text("Close")).on_press(Message::PanelClose(pane)),
  ]
  .align_y(Alignment::End);

  let title = container(text!("greeting"));


  TitleBar::new(title)
    .controls(Controls::new(content))
}

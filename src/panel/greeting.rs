use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::pane_grid::Axis;
use iced::widget::pane_grid::Pane;
use iced::widget::rich_text;
use iced::widget::row;
use iced::widget::space;
use iced::widget::span;
use iced::widget::text;

use crate::Mastary;
use crate::panel::Message;
use crate::panel::Panels;

pub fn view(
  state: &Mastary,
  pane: Pane,
  panel: &Panels,
  maximized: bool,
) -> iced::Element<'static, Message> {
  container(column![
    text!("Mastary v{}", crate::VERSION),
    space(),
    rich_text![
      "Create new ",
      span("vertical ")
      .link(Axis::Vertical)
      .color(state.theme.palette().success),
      "or ",
      span("horizontal ")
      .link(Axis::Horizontal)
      .color(state.theme.palette().success),
      "Panel."
    ]
    .on_link_click(move |axis| Message::PanelCreate(axis, pane))
  ])
  .style(container::bordered_box)
  .width(iced::Fill)
  .height(iced::Fill)
  .into()
}

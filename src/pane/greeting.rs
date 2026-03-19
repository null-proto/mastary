use iced::widget::column;
use iced::widget::container;
use iced::widget::pane_grid::Axis;
use iced::widget::pane_grid::Pane;
use iced::widget::rich_text;
use iced::widget::space;
use iced::widget::span;
use iced::widget::text;

use crate::pane::Message;
use crate::pane::PaneKind;

pub fn view(
  pane: Pane,
  _panel: &PaneKind,
  _maximized: bool,
) -> iced::Element<'static, Message> {
  container(column![
    text!("Mastary v{}", crate::VERSION),
    space(),
    rich_text![
      "Create new ",
      span("vertical ")
      .link(Axis::Vertical),
      // .color(state.theme.palette().success),
      "or ",
      span("horizontal ")
      .link(Axis::Horizontal),
      // .color(state.theme.palette().success),
      "Panel."
    ]
    .on_link_click(move |axis| Message::PaneCreate(axis, pane))
  ])
  .width(iced::Fill)
  .height(iced::Fill)
  .into()
}

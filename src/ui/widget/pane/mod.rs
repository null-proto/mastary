use iced::Alignment;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::Pixels;
use iced::widget::column;
use iced::widget::container;
use iced::widget::pane_grid::Pane;
use iced::widget::text;
use iced::widget::rich_text;
use iced::widget::pane_grid::Axis;
use iced::widget::button;

use crate::ui::State;
use crate::ui::UpdateMessage;

#[derive(Default, Clone, Debug)]
pub enum PaneWindow {
  #[default]
  Dummy,

  MasterFlow,
  // Flow,
  // Stash,
  // Filters,
  // Editor,
  // Viewer,
}

#[allow(unused_variables)]
impl<'a> PaneWindow {
  pub fn present(
    &self,
    state: &State,
    pane: &Pane,
    is_zen_mode: bool,
  ) -> Element<'static, UpdateMessage> {
    match self {
      Self::Dummy => Self::dummy_pane(pane.clone()),

      Self::MasterFlow => text!("MasterFlow not yet").into(),
    }
  }

  fn dummy_pane(pane: Pane) -> Element<'static, UpdateMessage> {
    container(
      container(
        column![
          text!("This is a dummy pane"),
          button(
            text("Create new")
            .size(Pixels::from(12))
            .width(Length::Fill)
          )
          .on_press(UpdateMessage::Ui(crate::ui::events::UiEvents::PaneCreateDummy(pane, Axis::Vertical) ) )
          .width(Length::Shrink)
        ]
        .width(Length::Shrink)
        .spacing(20)
        .padding(10),
      )
      .style(container::bordered_box),
    )
    .center(Length::Fill)
    .padding(10)
    .style(container::bordered_box)
    .into()
  }
}

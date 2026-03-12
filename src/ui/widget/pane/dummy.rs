use iced::Length;
use iced::Pixels;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::pane_grid::Axis;
use iced::widget::text;
use iced::{Element, widget::pane_grid::Pane};

use crate::ui::State;
use crate::ui::widget::pane::title::Title;
use crate::ui::{UpdateMessage, widget::pane::PaneMeta};

#[derive(Debug, Clone)]
pub struct Dummy {
  meta: PaneMeta,
  title: Title
}

impl Dummy {
  pub fn new() -> Self {
    Self {
      meta: PaneMeta {
        name: "Dummy".to_owned(),
      },
    }
  }

  pub fn view(
    &self,
    state: &State,
    pane: Pane,
    is_zen_mode: bool,
  ) -> Element<'static, UpdateMessage> {
    container(
      container(
        column![
          text!("This is a dummy pane"),
          button(
            text("Create new")
              .size(Pixels::from(12))
              .width(Length::Fill)
          )
          .on_press(UpdateMessage::Ui(
            crate::ui::events::UiEvents::PaneCreateDummy(pane, Axis::Vertical)
          ))
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

  pub fn title_bar(&self) -> Element<'static , > {
    self.title.view()
  }
}

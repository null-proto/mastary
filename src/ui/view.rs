#![allow(unused_imports)]

use iced::Alignment;
use iced::Element;
use iced::Fill;
use iced::Length;
use iced::widget::container;
use iced::widget::container::Style;
use iced::widget::mouse_area;
use iced::widget::pane_grid;
use iced::widget::row;
use iced::widget::space;
use iced::widget::text;

use crate::ui::UpdateMessage;
use crate::ui::events::UiEvents;
use crate::ui::widget::pane::PaneWindow;
use crate::ui::widget::sidebar::sidebar;

use super::events;

pub fn view(state: &super::State) -> Element<'_, UpdateMessage> {
  let pane_surface = pane_grid(&state.pane, |pane, pane_state, is_zen_mode| {
    pane_grid::Content::new(
      container(
        pane_state.present(&state, &pane, is_zen_mode)
      )
      .padding(10)
      .style(container::bordered_box)
    )
  })
  .spacing(10)
    .on_resize(8,|e| UpdateMessage::Ui(UiEvents::PaneResize(e)))
    ;

  mouse_area(container(row![ sidebar(&state) ,pane_surface,].spacing(10)).padding(10))
    .on_press(UpdateMessage::Ui(UiEvents::ContextMenuCloseAll))
    .into()
}

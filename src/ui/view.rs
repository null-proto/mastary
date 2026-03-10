use iced::{Element, widget::container::Style};
use iced::Fill;
use iced::Alignment;
use iced::Length;

use crate::ui::UpdateMessage;
use crate::ui::events::UiEvents;

use super::events;

#[allow(unused_imports)]
use iced::widget::{column, container, row, text};

pub fn view(state: &super::State) -> Element<'_, super::UpdateMessage> {
  container(row![
    side_bar(),
    modal(&state)
  ])
    .style(container::bordered_box)
    .padding(5)
    .into()
}

fn side_bar() -> Element<'static, super::UpdateMessage> {
  container(column![text("1")]).padding(10).into()
}

fn modal(state: &super::State) -> Element<'_, super::UpdateMessage> {
  container(
    pane(state)
  )
    .into()
}

fn pane(state: &super::State) -> Element<'_ , super::UpdateMessage> {
  use iced::widget::pane_grid;

  pane_grid(&state.pane, |pane , state, _| {
    pane_grid::Content::new(
      container(text!("pane {:?} is here!" , pane))
      .style(container::bordered_box)
      .padding(10)
      .width(Fill)
      .height(Fill)
    )
  })
  .on_drag(|e| 
    UpdateMessage::Ui(UiEvents::PaneDrag(e))
    )
  .into()

}

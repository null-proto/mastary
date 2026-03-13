use iced::Element;
use iced::Padding;
use iced::widget::button;
use iced::widget::container;
use iced::widget::column;
use iced::Alignment;
use iced::Length;

use crate::ui::State;
use crate::ui::UpdateMessage;

pub fn sidebar(_state : &State) -> Element<'_ , UpdateMessage> {

  container(
    column![
      button("1"),
      button("+"),
    ]
    .padding(Padding {
      top: 10.0,
      bottom: 10.0,
      right:5.0,
      left: 5.0,
    })
    .align_x(Alignment::Center)
    .spacing(5)
    .height(iced::Fill)
  )
    .align_x(Alignment::Center)
    .height(iced::Fill)
    .padding(5)
    .style(container::bordered_box)
    .into()
}

use iced::Element;
use iced::widget::container;
use iced::widget::text;

use crate::ui::UpdateMessage;

pub fn title() -> Element<'static, UpdateMessage> {
  container(text!("title")).into()
}

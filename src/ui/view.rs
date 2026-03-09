use iced::Element;
use iced::widget::{
  text
};

#[allow(unused_variables)]
pub fn view(state: &super::State ) -> Element<'_, super::UpdateMessage> {
  text("hello")
    .into()
}

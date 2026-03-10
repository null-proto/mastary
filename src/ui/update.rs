

#[allow(unused_variables)]
pub fn update(state : &mut super::State , message : super::UpdateMessage ) {
  use super::UpdateMessage::*;

  match message {
    Ui(event) => {
      tracing::debug!("EVENT UI: {:?}", event);
    },

    _else => {
      tracing::debug!("EVENT  {:?}", _else);
    }
  }
}

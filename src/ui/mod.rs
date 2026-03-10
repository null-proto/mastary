pub mod widget;
mod view;
mod update;
mod events;

pub use view::view;
pub use update::update;


#[derive(Clone , Debug)]
pub struct State {
  pane: iced::widget::pane_grid::State<PaneState>
}

impl State {
  pub fn new() -> Self {
    Self {
      pane: iced::widget::pane_grid::State::new(PaneState::Pane1).0,
    }
  }
}


#[derive(Default, Clone , Debug)]
pub enum UpdateMessage {
  Ui(events::UiEvents),

  #[default]
  UnConditional
}

impl UpdateMessage {
  pub fn new() -> Self {
    Self::UnConditional
  }
}

#[derive(Default, Clone , Debug)]
enum PaneState {
  #[default]
  Pane1,
  Pane2,
}

pub mod widget;
mod view;
mod update;
mod events;

pub use view::view;
pub use update::update;


use widget::pane::PaneWindow;

#[derive(Clone , Debug)]
pub struct State {
  pane: iced::widget::pane_grid::State<PaneWindow>,

  context_menu: bool
}

impl State {
  pub fn new() -> Self {
    Self {
      pane: iced::widget::pane_grid::State::new(PaneWindow::Dummy).0,
      context_menu: false
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


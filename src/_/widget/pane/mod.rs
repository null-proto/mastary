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

pub mod title;
pub mod dummy;

#[derive(Default, Clone, Debug)]
pub enum PaneWindow {
  #[default]
  Dummy(dummy::Dummy),

  MasterFlow,
  // Flow,
  // Stash,
  // Filters,
  // Editor,
  // Viewer,
}

#[derive(Clone, Debug)]
pub struct PaneMeta {
  pub name: String,
}

#[allow(unused_variables)]
impl<'a> PaneWindow {
  pub fn view(
    &self,
    state: &State,
    pane: &Pane,
    is_zen_mode: bool,
  ) -> Element<'static, UpdateMessage> {
    match self {
      Self::Dummy( dummy ) => dummy.view(state, pane.clone(), is_zen_mode),

      Self::MasterFlow => text!("MasterFlow not yet").into(),
    }
  }

  pub fn title_bar(
    &self,
    state: &State,
    pane: &Pane,
    is_zen_mode: bool,
  ) -> Element<'static, UpdateMessage> {
    match self {
      Self::Dummy( dummy ) => dummy.title_bar(),

      Self::MasterFlow => text!("MasterFlow not yet").into(),
    }
  }

  pub fn new_dummy_pane() -> Self {
    Self::Dummy(dummy::Dummy::new())
  }

}

use iced::widget::pane_grid::{Pane, ResizeEvent};

#[allow(unused_variables)]
pub fn update(state: &mut super::State, message: super::UpdateMessage) {
  use super::UpdateMessage::*;

  match message {
    Ui(event) => {
      tracing::debug!("EVENT UI: {:?}", event);
      ui_update(state, event);
    }

    _else => {
      tracing::debug!("EVENT  {:?}", _else);
    }
  }
}

fn ui_update(state: &mut super::State, message: super::events::UiEvents) {
  use super::events::UiEvents::*;
  use crate::ui::widget::pane::PaneWindow;

  match message {
    ContextMenuCreate => {
      state.context_menu = true;

      tracing::info!("{:?}", state);
    }

    ContextMenuCloseAll => {
      state.context_menu = false;
      tracing::info!("{:?}", state);
    }

    PaneCreateDummy(pane, axis) => {
      state.pane.split(axis, pane, PaneWindow::Dummy);
    }

    PaneResize( ResizeEvent { split , ratio} ) => {
      state.pane.resize(split, ratio);
    }

    PaneDrag(_) => {}
    _ => {}
  }
}

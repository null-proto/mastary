use iced::widget::pane_grid::Axis;
use iced::widget::pane_grid::DragEvent;
use iced::widget::pane_grid::Pane;
use iced::widget::pane_grid::ResizeEvent;

#[derive(Clone, Debug, Default)]
pub enum UiEvents {
  PaneDrag(DragEvent),

  ContextMenuCreate,
  ContextMenuCloseAll,

  PaneCreateDummy(Pane, Axis),
  PaneClose(Pane),
  PaneResize(ResizeEvent),

  #[default]
  UnConditionalUpdate,
}

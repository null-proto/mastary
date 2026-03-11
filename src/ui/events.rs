use iced::widget::pane_grid::Axis;
use iced::widget::pane_grid::Pane;
use iced::widget::pane_grid::ResizeEvent;
use iced::widget::pane_grid::DragEvent;

#[derive(Clone, Debug, Default)]
pub enum UiEvents {
  PaneDrag(DragEvent),

  ContextMenuCreate,
  ContextMenuCloseAll,

  PaneCreateDummy(Pane, Axis),
  PaneClose(Pane),
  PaneResize(ResizeEvent),
  PaneDragges(DragEvent),

  #[default]
  UnConditionalUpdate,
}

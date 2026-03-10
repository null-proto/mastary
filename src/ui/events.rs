use iced::widget::pane_grid;


#[derive(Clone, Debug , Default)]
pub enum UiEvents {
  PaneDrag(pane_grid::DragEvent),

  #[default]
  UnConditionalUpdate
}

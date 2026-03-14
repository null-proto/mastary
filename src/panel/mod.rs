use iced::widget::container;
use iced::widget::pane_grid;
use iced::widget::pane_grid::Pane;
use iced::widget::pane_grid::Axis;
use iced::widget::pane_grid::DragEvent;
use iced::widget::pane_grid::ResizeEvent;
use iced::widget::text;
use iced::Alignment;

#[derive(Debug, Default, Clone)]
pub enum Panels {
  #[default]
  Greetings,
}

#[derive(Debug, Clone)]
pub enum Message {
  PanelDrag(DragEvent),
  PanelsResize(ResizeEvent),
  PanelClose,
  PanelCreate(Axis, Pane),
  PanelMaximized,
  PanelUnMaximized,
}

impl Panels {
  pub fn new() -> Self {
    Panels::Greetings
  }

  pub fn view(state: &crate::Mastary) -> iced::Element<'_, Message> {
    let surface = pane_grid(&state.panels, |id, panel, maximized| {
      pane_grid::Content::new(
        container(text!("--"))
          .padding(10)
          .style(container::bordered_box)
          .width(iced::FillPortion(400))
          ,
      )
      .title_bar(pane_grid::TitleBar::new(panel.title()))
    })
    .spacing(10);

    surface.into()
  }

  pub fn title(&self) -> iced::Element<'_, Message> {
    text("hello").into()
  }

  pub fn update(state: &mut crate::Mastary, msg: Message) {
    match msg {
      Message::PanelCreate(axis, pane) => {
        state.panels.split(axis, pane, Panels::Greetings );
      }

      Message::PanelsResize(ResizeEvent { split, ratio }) => {
        state.panels.resize(split, ratio);
      }

      Message::PanelDrag(DragEvent::Dropped { pane, target }) => {
        state.panels.drop(pane, target);
      }

      Message::PanelClose => {}

      _ => {}
    }
  }
}

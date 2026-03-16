// use iced::Alignment;
use iced::widget::container;
use iced::widget::pane_grid;
use iced::widget::pane_grid::Axis;
use iced::widget::pane_grid::DragEvent;
use iced::widget::pane_grid::Pane;
use iced::widget::pane_grid::ResizeEvent;
use iced::widget::text;

mod greeting;
mod title;

#[derive(Debug, Default, Clone)]
pub enum Panels {
  #[default]
  Greetings,
}

#[derive(Debug, Clone)]
pub enum Message {
  PanelDrag(DragEvent),
  PanelsResize(ResizeEvent),
  PanelClose(Pane),
  PanelCreate(Axis, Pane),
  PanelFocusChange(Pane),
  PanelMaximized,
  PanelUnMaximized,
}

impl<'a> Panels {
  fn present(
    &'a self,
    state: &'a crate::Mastary,
    id: Pane,
    maximized: bool,
  ) -> pane_grid::Content<'a, Message> {
    match self {
      Panels::Greetings => pane_grid::Content::new(greeting::view(state, id, self, maximized))
        .style(container::bordered_box)
        .title_bar(self.title(state, id ,maximized)),
    }
  }

  fn title(&'a self, state: &'a crate::Mastary, id: Pane ,maximized: bool) -> pane_grid::TitleBar<'a, Message> {
    match self {
      Panels::Greetings => {
        title::view(state, id, self, maximized)
      },
    }
  }

  pub fn view(state: &'a crate::Mastary) -> iced::Element<'a, Message> {
    let surface = pane_grid(&state.panels, |id, panel, maximized| {
      panel.present(state, id, maximized)
    })
    .spacing(6)
    .on_resize(8, Message::PanelsResize)
    .on_click(Message::PanelFocusChange)
    .on_drag(Message::PanelDrag);

    container(surface)
      .padding(6)
      .into()
  }

  pub fn update(state: &mut crate::Mastary, msg: Message) {
    match msg {
      Message::PanelCreate(axis, pane) => {
        state.panels.split(axis, pane, Panels::Greetings);
      }

      Message::PanelsResize(ResizeEvent { split, ratio }) => {
        state.panels.resize(split, ratio);
      }

      Message::PanelDrag(DragEvent::Dropped { pane, target }) => {
        state.panels.drop(pane, target);
      }

      Message::PanelFocusChange(_pane) =>{}

      Message::PanelClose(pane) => {
        state.panels.close(pane);
      }

      _ => {}
    }
  }
}

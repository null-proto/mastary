use iced::widget::container;
use iced::widget::pane_grid;
use iced::widget::pane_grid::Axis;
use iced::widget::pane_grid::DragEvent;
use iced::widget::pane_grid::Pane;
use iced::widget::pane_grid::State;
use iced::widget::pane_grid::ResizeEvent;


mod greeting;
mod title;

#[derive(Clone,Debug)]
pub struct PaneController {
  pub(crate) state: State<PaneKind>
}

#[derive(Debug, Default, Clone)]
pub enum PaneKind {
  #[default]
  Greeting,
}

#[derive(Debug, Clone)]
pub enum Message {
  PaneDrag(DragEvent),
  PanesResize(ResizeEvent),
  PaneClose(Pane),
  PaneCreate(Axis, Pane),
  PaneFocusChange(Pane),
  PaneMaximized,
  PaneUnMaximized,
}

impl<'a> PaneController {
  pub fn new() -> Self {
    let (state , _pane_id) = State::new(PaneKind::Greeting);

    Self {
      state
    }
  }

  pub fn view(&'a self) -> iced::Element<'a, Message> {
    let surface = pane_grid(&self.state, |id, panel, maximized| {
      panel.view( id, maximized)
    })
    .spacing(6)
    .on_resize(8, Message::PanesResize)
    .on_click(Message::PaneFocusChange)
    .on_drag(Message::PaneDrag);

    container(surface)
      .padding(6)
      .into()

  }


  pub fn update(&mut self, msg: Message) {
    match msg {
      Message::PaneCreate(axis, pane) => {
        self.state.split(axis, pane, PaneKind::Greeting);
      }

      Message::PanesResize(ResizeEvent { split, ratio }) => {
        self.state.resize(split, ratio);
      }

      Message::PaneDrag(DragEvent::Dropped { pane, target }) => {
        self.state.drop(pane, target);
      }

      Message::PaneFocusChange(_pane) =>{}

      Message::PaneClose(pane) => {
        self.state.close(pane);
      }

      _ => {}
    }
  }
}


impl<'a> PaneKind {
  pub fn view(
    &'a self,
    id: Pane,
    maximized: bool,
  ) -> pane_grid::Content<'a, Message> {
    match self {
      PaneKind::Greeting => pane_grid::Content::new(greeting::view(id, self, maximized))
        .style(container::bordered_box)
        .title_bar(self.title(id)),
    }
  }

  fn title(&'a self,id: Pane) -> pane_grid::TitleBar<'a, Message> {
    match self {
      PaneKind::Greeting => {
        title::view(id)
      },
    }
  }

}

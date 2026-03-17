use iced::window::Id;

use crate::pane;
use crate::sidebar;

#[derive(Clone, Debug)]
pub struct Window {
  pub(crate) id: Id,
  pub(crate) is_presended: bool,
  inner: WindowKind,
}

#[derive(Clone, Debug)]
pub enum Message {
  SideBar(sidebar::Message),
  Pane(pane::Message),
}

#[derive(Clone, Debug)]
enum WindowKind {
  MainWindow {
    sidebar: sidebar::SideBar,
    pane: pane::PaneController,
  },
}

impl<'a> Window {
  pub fn default_main(id: Id) -> Self {
    let kind = WindowKind::MainWindow {
      sidebar: sidebar::SideBar::new(),
      pane: pane::PaneController::new(),
    };

    Self { id, is_presended: false ,inner: kind }
  }

  pub fn view(self: &'a Self, theme: &crate::Theme) -> iced::Element<'a, Message> {
    match &self.inner {
      WindowKind::MainWindow { sidebar, pane } => iced::widget::row![
        sidebar.view().map(Message::SideBar),
        pane.view().map(Message::Pane)
      ]
      .into(),
    }
  }

  pub fn update(&mut self, msg: Message) {
    match msg {
      Message::Pane(msg) => match &mut self.inner {
        WindowKind::MainWindow { sidebar: _, pane } => pane.update(msg),
      },

      Message::SideBar(msg) => match &mut self.inner {
        WindowKind::MainWindow { sidebar, pane: _ } => sidebar.update(msg),
      },
    }
  }
}

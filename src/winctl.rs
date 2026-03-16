use iced::window::Event;
use iced::window::Id;
use std::collections::HashSet;

use crate::window;
use crate::window::Window;

#[derive(Clone, Debug)]
pub struct WindowController {
  ids: HashSet<Id>,
  window: Vec<Window>
}

#[derive(Clone, Debug)]
pub enum Message {
  WindowEvent(Id, Event),
  UI(window::Message)
}

impl WindowController {
  pub fn new() -> Self {
    Self {
      ids: Default::default(),
      window: Default::default()
    }
  }


  pub fn view(&self, id : Id) -> iced::Element<'_, Message> {
    if let Some(window) = self.window.iter().find(|w| w.id == id) {
      window.view().map(Message::UI)
    } else {
      // FATAL: window id doesn't exists in the state
      iced::widget::space().into()
    }
  }

  pub fn update(&mut self, msg: Message) {
    match msg {
      Message::UI(_event) => {
      }

      Message::WindowEvent(id, event) => {
        match event {
          iced::window::Event::Opened {
            position: _,
            size: _,
          } => {
            tracing::info!("new window {}", id);
            self.ids.insert(id);
          }

          iced::window::Event::Closed => {
            self.ids.remove(&id);
            tracing::info!("window {} closed", id);
          }

          _ => {}
        }
      }
    }
  }
}

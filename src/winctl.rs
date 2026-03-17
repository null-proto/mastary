use iced::Subscription;
use iced::window::Event;
use iced::window::Id;
use std::collections::HashSet;

use crate::window;
use crate::window::Window;


#[derive(Clone, Debug)]
pub struct WindowController {
  ids: HashSet<Id>,
  window: Vec<Window>,
}

#[derive(Clone, Debug)]
pub enum Message {
  MainWindowCreate(Id),
  MainWindowDestroy(Id),
  WindowEvent((Id, Event) ),
  Interface(Id ,window::Message),
}

impl WindowController {
  pub fn create_main_window() -> iced::Task<Message> {
    let (_window_id_, window_open_task) = iced::window::open(iced::window::Settings::default());
    window_open_task.map(Message::MainWindowCreate)
  }

  pub fn sunscribe_window_events() -> Subscription<Message> {
    iced::window::events().map(Message::WindowEvent)
  }

  pub fn new() -> Self {
    Self {
      ids: Default::default(),
      window: Default::default(),
    }
  }

  pub fn view(&self, id: Id) -> iced::Element<'_, Message> {
    if let Some(w) = &self.window.iter().find(|wi| wi.id == id) {
      w.view().map(move |e| Message::Interface(id , e))
    } else {
      // FATAL: window id doesn't exists in the state
      iced::widget::container(
        iced::widget::column![
          iced::widget::text(format!("FATAL : Window not found for ID({})", id)),
          iced::widget::text(format!("{:?}", self))
        ]
        .spacing(10),
      )
      .center(iced::Fill)
      .style(iced::widget::container::bordered_box)
      .padding(20)
      .into()
    }
  }

  pub fn update(&mut self, msg: Message) {
    match msg {
      Message::Interface(id,event) => {
        if let Some(w) = self.window.iter_mut().find(|wi| wi.id == id) {
          w.update(event);
        } else {
          tracing::error!("FATAL : Window not found for ID({}), {:?}",id , event );
        }
      }

      Message::MainWindowCreate(id) => {
        tracing::info!("new main window {}", id);
        let w = Window::default_main(id);
        self.window.push(w);
      }

      Message::MainWindowDestroy(id) => {
      //   self.active_window = None;
        tracing::info!("main window {} closed", id);
      }

      Message::WindowEvent((id, event)) => match event {
        iced::window::Event::Opened {
          position: _,
          size: _,
        } => {
          tracing::info!("new window {}", id);

          // window creation is explicit
          //
          // let w = window::Window::default_main(id);
          // self.window.push(w);

          self.ids.insert(id);
        }

        iced::window::Event::Closed => {
          self.ids.remove(&id);
          tracing::info!("window {} closed", id);
        }

        // iced::window::Event::Focused => {
        //   self.active_window = self.window.iter_mut().find(|wi| wi.id == id);
        // }
        //
        _ => {}
      },
    }
  }
}

use iced::widget::container::transparent;
use tracing::info;

use iced::Settings;
use iced::Task;
use iced::Theme;
use iced::window::Event as WinEvent;
use iced::window::Id;

use crate::window;
use crate::window::Window;

pub fn init_ui() {
  info!("initializing ui ...");
  let default_font = iced::Font::with_name("VictorMono Nerd Font Mono");

  match iced::daemon(move || Mastary::new(), Mastary::update, Mastary::view)
    .title(Mastary::title)
    .theme(Mastary::theme)
    .subscription(Mastary::subscribe)
    .scale_factor(Mastary::scale_factor)
    .default_font(default_font)
    .run()
  {
    Ok(_) => {
      tracing::info!("daemon completes its job");
    }

    Err(e) => {
      tracing::error!("FATAL: {}", e.to_string());
      tracing::debug!("FATAL: {:?}", e);
    }
  }

  // TODO: daemon should exit if all windows're closed
  info!("exiting ...");
}

#[derive(Debug, Clone)]
pub struct Mastary {
  theme: Theme,
  theme_ext: crate::theme::Theme,
  scale_factor: f32,
  settings: Settings,
  title: String,
  window: Vec<Window>,
}

#[derive(Debug, Clone)]
pub enum Message {
  InitCompleted,
  FontLoaded(bool),
  MainWindowCreate(Id),
  MainWindowDestroy(Id),
  WindowEvent((Id, WinEvent)),
  Interface(Id, window::Message),
}

impl Default for Mastary {
  fn default() -> Self {
    let (theme_ext, theme) = crate::theme::Theme::cattppuccin_mocha();

    Self {
      theme,
      theme_ext,
      scale_factor: 1.0,
      settings: Settings::default(),
      title: String::default(),
      window: Default::default(),
    }
  }
}

impl Mastary {
  pub fn new() -> (Self, iced::Task<Message>) {
    let mastary = Mastary::default();

    let mut tasks: Vec<Task<Message>> = vec![];


    // default settings
    let settings = iced::window::Settings {
      platform_specific : iced::window::settings::PlatformSpecific {
        application_id: "".to_string(),
        override_redirect: true,
      },
      size: iced::Size::new(600.0, 800.0),
      min_size: Some(iced::Size::new(400.0, 800.0)),
      fullscreen: false,
      decorations: false,
      transparent: true,
      exit_on_close_request: false,
      ..Default::default()
    };

    tracing::trace!("using window settings: {:#?}" , settings);

    let (_window_id, window_create) = iced::window::open(settings);

    let icon_font = iced::font::load(include_bytes!( "../fonts/VictorMonoNerdFont-Bold.ttf" ));

    tasks.push(window_create.map(Message::MainWindowCreate));
    tasks.push(icon_font.map(|r|Message::FontLoaded(r.is_ok())));

    (mastary, Task::batch(tasks))
  }

  pub fn view(&self, id: Id) -> iced::Element<'_, Message> {
    if let Some(w) = &self.window.iter().find(|wi| wi.id == id) {
      w.view(&self.theme_ext).map(move |e| Message::Interface(id, e))
    } else {
      // FATAL: window id doesn't exists in the state
      //
      // this won't happen in normal case
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
      Message::InitCompleted => {}

      Message::FontLoaded(r) => {
        tracing::info!("icon font loaded: {}", r);
      }

      Message::Interface(id, event) => {
        if let Some(w) = self.window.iter_mut().find(|wi| wi.id == id) {
          w.update(event);
        } else {
          tracing::error!("FATAL : Window not found for ID({}), {:?}", id, event);
        }
      }

      Message::MainWindowCreate(id) => {
        tracing::info!("new main window {}", id);
        let w = Window::default_main(id);
        self.window.push(w);
      }

      Message::MainWindowDestroy(id) => {
        tracing::info!("main window {} closed", id);
      }

      Message::WindowEvent((id, event)) => match event {
        WinEvent::Opened {
          position: _,
          size: _,
        } => {
          tracing::info!("new window {}", id);
          if let Some(window_index) = self.window.iter().enumerate().find_map(|(i,w)| if w.id==id { Some(i) } else { None } ) {
            self.window[window_index].is_presended = true;
          }
        }

        WinEvent::Closed => {
          if let Some(window_index) = self.window.iter().enumerate().find_map(|(i,w)| if w.id==id { Some(i) } else { None } ) {
            self.window.remove(window_index);
          }
          tracing::info!("window {} closed", id);
        }

        WinEvent::Focused => {
          // TODO: fix this
          if let Some(id_) = self.window.get(0) && id_.id != id {
            if let Some(f_window_index) = self.window.iter().enumerate().find_map(|(i,w)| if w.id==id { Some(i) } else { None } ) {
              tracing::debug!("window swapping, index ({}) to 0",id);
              let f_window = self.window.remove(f_window_index);
              self.window.insert(0, f_window);
            }
          }
        }

        _ => {}
      },
    }
  }

  pub fn title(&self, _id: Id) -> String {
    self.title.clone()
  }

  pub fn subscribe(&self) -> iced::Subscription<Message> {
    let mut subs = vec![];
    subs.push(iced::window::events().map(Message::WindowEvent));

    iced::Subscription::batch(subs)
  }

  pub fn theme(&self, _id: Id) -> iced::Theme {
    self.theme.clone()
  }

  pub fn settings(&self) -> iced::Settings {
    self.settings.clone()
  }

  pub fn scale_factor(&self, _id: Id) -> f32 {
    self.scale_factor
  }
}

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

  // iced::application( || { ui::State::new() } , ui::update, ui::view)
  //   .decorations(false)
  //   .default_font(font)
  //   .theme(iced::Theme::CatppuccinMocha)
  //   // .subscription(AppMain::subscribe)
  //   .run()
  //   .unwrap();

  // font setup seems skeptic
  //
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

  // it blocks
  //
  info!("exiting ...");
}

#[derive(Debug, Clone)]
pub struct Mastary {
  theme: Theme,
  theme_ext: crate::theme::Theme,
  global_scale_factor: f32,
  settings: Settings,
  title: String,
  window: Vec<Window>,
}

#[derive(Debug, Clone)]
pub enum Message {
  InitCompleted,
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
      global_scale_factor: 1.2f32,
      settings: Settings::default(),
      title: String::default(),
      window: Default::default(),
    }
  }
}

#[allow(unused)]
impl Mastary {
  pub fn new() -> (Self, iced::Task<Message>) {
    let mastary = Mastary::default();

    let mut tasks: Vec<Task<Message>> = vec![];

    let (_window_id, window_create) = iced::window::open(iced::window::Settings::default());

    tasks.push(window_create.map(Message::MainWindowCreate));

    (mastary, Task::batch(tasks))
  }

  pub fn view(&self, id: Id) -> iced::Element<'_, Message> {
    if let Some(w) = &self.window.iter().find(|wi| wi.id == id) {
      w.view().map(move |e| Message::Interface(id, e))
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
      Message::InitCompleted => {}

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
          // window creation is explicit
        }

        WinEvent::Closed => {
          tracing::info!("window {} closed", id);
        }

        _ => {}
      },
    }
  }

  pub fn title(&self, id: Id) -> String {
    self.title.clone()
  }

  pub fn subscribe(&self) -> iced::Subscription<Message> {
    let mut subs = vec![];
    subs.push(iced::window::events().map(Message::WindowEvent));

    iced::Subscription::batch(subs)
  }

  pub fn theme(&self, id: Id) -> iced::Theme {
    self.theme.clone()
  }

  pub fn settings(&self) -> iced::Settings {
    self.settings.clone()
  }

  pub fn scale_factor(&self, id: Id) -> f32 {
    self.global_scale_factor
  }
}

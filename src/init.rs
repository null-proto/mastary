use tracing::info;

use iced::Settings;
use iced::Task;
use iced::Theme;
use iced::window::Event as WinEvent;
use iced::window::Id as WinID;

use crate::winctl;
use crate::winctl::WindowController;

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
  windows: WindowController,
  theme: Theme,
  theme_ext: crate::theme::Theme,
  global_scale_factor: f32,
  settings: Settings,
  title: String,
}

#[derive(Debug, Clone)]
pub enum Message {
  InitCompleted,
  Window(WinID, WinEvent),
  WindowController(winctl::Message)
}

impl Default for Mastary {
  fn default() -> Self {
    let (theme_ext, theme) = crate::theme::Theme::cattppuccin_mocha();

    Self {
      windows: WindowController::new(),
      theme,
      theme_ext,
      global_scale_factor: 1.2f32,
      settings: Settings::default(),
      title: String::default(),
    }
  }
}

#[allow(unused)]
impl Mastary {
  pub fn new() -> (Self, iced::Task<Message>) {
    let mastary = Mastary::default();

    let mut tasks: Vec<Task<Message>> = vec![];

    tasks.push( winctl::WindowController::create_main_window().map(|id| Message::WindowController(id)));

    (mastary, Task::batch(tasks))
  }

  pub fn view(&self, id: WinID) -> iced::Element<'_, Message> {
    self.windows.view(id).map(Message::WindowController)
  }

  pub fn update(&mut self, msg: Message) {
    match msg {
      Message::Window(window_id, window_event) => {
        tracing::trace!("window event: {},{:?}", window_id , window_event);
      }

      Message::WindowController(event) => {
        self.windows.update(event);
      }

      Message::InitCompleted => {}
    }
  }

  pub fn title(&self, id: WinID) -> String {
    self.title.clone()
  }

  pub fn subscribe(&self) -> iced::Subscription<Message> {
    let mut subs = vec![];
    subs.push(WindowController::sunscribe_window_events().map(Message::WindowController) );

    iced::Subscription::batch(subs)
  }

  pub fn theme(&self, id: WinID) -> iced::Theme {
    self.theme.clone()
  }

  pub fn settings(&self) -> iced::Settings {
    self.settings.clone()
  }

  pub fn scale_factor(&self, id: WinID) -> f32 {
    self.global_scale_factor
  }
}

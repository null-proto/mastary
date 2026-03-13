use tracing::info;

use std::collections::HashSet as Set;
use std::default;

use iced::Settings;
use iced::Task;
use iced::Theme;
use iced::window::Event as WinEvent;
use iced::window::Id as WinID;

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
    .scale_factor(Mastary::scale)
    .default_font(default_font)
    .run() {

      Ok(_) => {}

      Err(e) => {
        tracing::error!("FATAL: {}", e.to_string());
        tracing::debug!("FATAL: {:?}", e);
      }
    }


  // it blocks
  info!("exiting ...");
}

#[derive(Debug, Clone)]
struct Mastary {
  windows: Set<iced::window::Id>,
  theme: Theme,
  global_scal_factor: f32,

  settings: Settings,
  title: String,
}

#[derive(Debug, Clone)]
enum Message {
  InitCompleted,
  Window(WinID, WinEvent),
}

impl Default for Mastary {
  fn default() -> Self {
    let font = iced::Font::with_name("VictorMono Nerd Font Mono");

    Self {
      windows: Set::new(),
      theme: Theme::Nord,
      global_scal_factor: 1.2f32,
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

    let (window_id, window_open_task) = iced::window::open(iced::window::Settings::default());

    tasks.push(window_open_task.map(|_| Message::InitCompleted));

    (mastary, Task::batch(tasks))
  }

  pub fn view(&self, id: WinID) -> iced::Element<'_, Message> {
    iced::widget::column!["hello"].into()
  }

  pub fn update(&mut self, msg: Message) {
    match msg {
      Message::Window(window_id, window_event) => {
        update_window_events(self, window_id, window_event);
      }

      Message::InitCompleted => {}
    }
  }

  pub fn title(&self, id: WinID) -> String {
    self.title.clone()
  }

  pub fn subscribe(&self) -> iced::Subscription<Message> {
    iced::Subscription::none()
  }

  pub fn theme(&self, id: WinID) -> iced::Theme {
    self.theme.clone()
  }

  pub fn settings(&self) -> iced::Settings {
    self.settings.clone()
  }

  pub fn scale(&self, id: WinID) -> f32 {
    self.global_scal_factor
  }
}

fn update_window_events(state: &mut Mastary, window_id: WinID, window_event: WinEvent) {
  match window_event {
    iced::window::Event::Opened { position:_, size:_ } => {
      tracing::info!("new window {}", window_id);
      state.windows.insert(window_id);
    }

    iced::window::Event::Closed => {
      state.windows.remove(&window_id);
      tracing::info!("window {} closed", window_id);
    }

    _ => {}
  }
}

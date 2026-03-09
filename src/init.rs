use tracing::info;

use crate::ui;


pub fn init_ui() {
  info!("initializing ui ...");
  let font = iced::Font::with_name("VictorMono Nerd Font Mono");

  // iced::daemon(|| AppMain::default(), AppMain::update, AppMain::view)

  iced::application( || { ui::State::new() } , ui::update, ui::view)
    .decorations(false)
    .default_font(font)
    .theme(iced::Theme::CatppuccinMocha)
    // .subscription(AppMain::subscribe)
    .run()
    .unwrap();
  info!("exiting ...");

}

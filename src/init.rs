use tracing::info;

use crate::ui;


pub fn init_ui() {
  info!("initializing ui ...");
  let mut font = iced::Font::with_name("VictorMono Nerd Font Mono");
  font.style = iced::font::Style::Normal;
  font.weight = iced::font::Weight::Semibold;


  iced::application( || { ui::State::new() } , ui::update, ui::view)
    .decorations(false)
    .default_font(font)
    .theme(iced::Theme::CatppuccinMocha)
    // .subscription(AppMain::subscribe)
    .run()
    .unwrap();
  info!("exiting ...");

}

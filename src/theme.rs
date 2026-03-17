use iced::Color;
use iced::color;
use iced::theme::Theme as IcedTheme;
use iced::theme::Palette;

#[derive(Clone, Debug)]
pub struct Theme {
  pub background: Color,
  pub surface0: Color,

  pub accent: Color,
  pub accent_alt: Color,

  pub success: Color,
  pub warning: Color,
  pub error: Color,
  pub border: Color,

  pub text: Color,
  pub text_muted: Color,
  pub text_selection: Color,

  pub black: Color,
  pub red: Color,
  pub green: Color,
  pub yellow: Color,
  pub blue: Color,
  pub megenta: Color,
  pub cyan: Color,
  pub white: Color,
  pub orange: Color,
}


impl Theme {
  pub fn get_iced(&self) -> IcedTheme {
    let palette = Palette {
      background: self.background,
      text: self.text,
      primary: self.accent,
      success: self.success,
      warning: self.warning,
      danger: self.error
    };

    IcedTheme::custom("Mocha", palette)
  }


  pub fn cattppuccin_mocha() -> (Theme, IcedTheme) {
    let theme = Theme {
      background: color!(0x181825),
      surface0: color!(0x1e1e2e),

      accent: color!(0xcba6f7),
      accent_alt: color!(0x74c7ec),
      success: color!(0x89b4fa),
      warning: color!(0xf9e2af),
      error: color!(0xf38ba8),
      border: color!(0x45475a),

      text: color!(0xcdd6f4),
      text_muted: color!(0x11111b),
      text_selection: color!(0xf5e0dc),

      black: color!(0x181825),
      red: color!(0xf38ba8),
      green: color!(0xa6e3a1),
      yellow: color!(0xf9e2af),
      blue: color!(0x89b4fa),
      megenta: color!(0xeba0ac),
      cyan: color!(0x89dceb),
      white: color!(0xdce0e8),
      orange: color!(0xef9f76),
    };

    let iced = theme.get_iced();

    (theme, iced)
  }
}

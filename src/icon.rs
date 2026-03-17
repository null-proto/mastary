use iced::Element;



pub enum Icons {
  None
}

impl<'a> Icons {
  pub fn text<T>(&'a self) -> Element<'a,T> {
    iced::widget::text(self.as_ref()).into()
  }
}

impl AsRef<str> for Icons {
  fn as_ref(&self) -> &str {
    use Icons::*;

    match self {
      None => " "
    }
  }
}

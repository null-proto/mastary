use iced::widget::text;


#[derive(Debug,Default,Clone)]
pub enum Panels {

  #[default]
  Greetings
}

#[derive(Debug,Clone)]
pub enum Message {

}


impl Panels {
  pub fn new() -> Self {
    Panels::Greetings
  }


  pub fn view(&self , state: &crate::Mastary) -> iced::Element<'_ , Message> {
    text("hello").into()
  }

  pub fn update(&self , msg: Message) {
  }
}

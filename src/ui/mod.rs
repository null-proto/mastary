pub mod widget;
mod view;
mod update;

pub use view::view;
pub use update::update;


#[derive(Default, Clone , Debug)]
pub struct State {

}

impl State {
  pub fn new() -> Self {
    Self {}
  }
}


#[derive(Default, Clone , Debug)]
pub enum UpdateMessage {

  #[default]
  UnConditional
}

impl UpdateMessage {
  pub fn new() -> Self {
    Self::UnConditional
  }
}



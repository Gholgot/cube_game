use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Player;

#[gdnative::methods]
impl Player {
    fn new(_owner: &Node) -> Self {
        Player
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        //! To remove
        godot_print!("Player class initialized.")
    }

    #[export]
    fn _input(&self, _owner: &Node, event: gdnative::Ref<InputEvent>) {
      let event = unsafe { event.assume_safe() };
      if event.is_action("move_forward") {
        godot_print!("Moving forward")
      }
    }
}
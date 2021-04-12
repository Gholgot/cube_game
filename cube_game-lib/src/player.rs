use gdnative::prelude::*;
use gdnative::api::Spatial;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Player {
  speed: f32
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &Spatial) -> Self {
        Player{
          speed: 0.2
        }
    }

    #[export]
    fn _ready(&self, _owner: &Spatial) {
        //! To remove
        godot_print!("Player class initialized.")
    }

    #[export]
    fn _input(&self, owner: &Spatial, event: gdnative::Ref<InputEvent>) {
      let event = unsafe { event.assume_safe() };
      if event.is_action_pressed("move_forward", true) {
        self._move(owner, "forward".to_string())
      }
      if event.is_action_pressed("move_backward", true) {
        self._move(owner, "backward".to_string())
      }
      if event.is_action_pressed("move_left", true) {
        self._move(owner, "left".to_string())
      }
      if event.is_action_pressed("move_right", true) {
        self._move(owner, "right".to_string())
      }
    }

    #[export]
    fn _move(&self, owner: &Spatial, direction: String) {
      match direction {
        _ if direction == "forward" => owner.translate(- owner.global_transform().basis.z() * self.speed),
        _ if direction == "backward" => owner.translate(owner.global_transform().basis.z()),
        _ if direction == "left" => owner.translate(- owner.global_transform().basis.x()),
        _ if direction == "right" => owner.translate(owner.global_transform().basis.x()),
        _ => godot_print!("Doing nothing")
      }
    }
}
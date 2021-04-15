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
          speed: 0.3
        }
    }

    #[export]
    fn _ready(&self, _owner: &Spatial) {
        //! To remove
        godot_print!("Player class initialized.")
    }

    #[export]
    fn _physics_process(&self, owner: &Spatial, delta: f32) {
      self._process_input(owner, delta);

    }

    #[export]
    fn _process_input(&self, owner: &Spatial, delta: f32) {
      let input = Input::godot_singleton();
      let dir = Vector3::new(0.0,0.0,0.0);

      let cam_xform = owner.find_node("Camera", true, true);

      if input.is_action_pressed("move_forward") {
        self._move(owner, "forward".to_string())
      }
      if input.is_action_pressed("move_backward") {
        self._move(owner, "backward".to_string())
      }
      if input.is_action_pressed("move_left") {
        self._move(owner, "left".to_string())
      }
      if input.is_action_pressed("move_right") {
        self._move(owner, "right".to_string())
      }
    }

    #[export]
    fn _move(&self, owner: &Spatial, direction: String) {
      match direction {
        _ if direction == "forward" => owner.translate(- owner.global_transform().basis.z() * self.speed),
        _ if direction == "backward" => owner.translate(owner.global_transform().basis.z() * self.speed),
        _ if direction == "left" => owner.translate(- owner.global_transform().basis.x() * self.speed),
        _ if direction == "right" => owner.translate(owner.global_transform().basis.x() * self.speed),
        _ => godot_print!("Doing nothing")
      }
    }

    #[export]
    fn _get_player_representation(&self, owner: &Spatial) -> Option<Ref<Node>> {
      owner.get_child(0)
    }
}
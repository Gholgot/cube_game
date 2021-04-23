use gdnative::prelude::*;
use gdnative::api::RigidBody;

#[derive(NativeClass)]
#[inherit(RigidBody)]
pub struct Player {
  speed: f32,
  gravity: f32,
  velocity: Vector3,
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &RigidBody) -> Self {
        Player{
          speed: 0.2,
          gravity: -24.0,
          velocity: Vector3::new(0.0,0.0,0.0)
        }
    }

    #[export]
    fn _physics_process(&self, owner: &RigidBody, _delta: f32) {
      //owner.translate(owner.global_transform().basis.y() * (_delta * self.gravity));
      self._process_input(owner, _delta);
    }

    #[export]
    fn _process_input(&self, owner: &RigidBody, _delta: f32) {
      let input = Input::godot_singleton();
      let _dir = Vector3::new(0.0,0.0,0.0);

      let _cam_xform = owner.find_node("Camera", true, true);

      if input.is_action_pressed("move_forward") {
        owner.translate(- owner.global_transform().basis.z() * self.speed)
      }
      if input.is_action_pressed("move_backward") {
        owner.translate(owner.global_transform().basis.z() * self.speed)
      }
      if input.is_action_pressed("move_left") {
        owner.translate(- owner.global_transform().basis.x() * self.speed)
      }
      if input.is_action_pressed("move_right") {
        owner.translate(owner.global_transform().basis.x() * self.speed)
      }
    }

    #[export]
    fn _get_player_representation(&self, owner: &RigidBody) -> Option<Ref<Node>> {
      owner.get_child(0)
    }
}
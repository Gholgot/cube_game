use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Terrain;

#[gdnative::methods]
impl Terrain {
    fn new(_owner: &Node) -> Self {
        Terrain
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        //! To remove
        godot_print!("Terrain class initialized.")
    }
}
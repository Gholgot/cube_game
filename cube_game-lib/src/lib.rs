use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
struct Main;

#[gdnative::methods]
impl Main {
    fn new(_owner: &Node) -> Self {
        Main
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("hello, world.")
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Main>();
}

godot_init!(init);
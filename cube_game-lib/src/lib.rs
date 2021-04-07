use gdnative::prelude::*;

mod player;
mod terrain;

fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<terrain::Terrain>();
}

godot_init!(init);
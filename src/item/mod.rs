use raylib::ffi::Vector2;

pub trait Item {
    fn get_name(&self) -> String;
}

pub struct MapItem {
    pub position: Vector2,
    pub item: Box<dyn Item>,
}

unsafe impl Send for MapItem {}

pub struct Pickaxe {
    
}

impl Item for Pickaxe {
    fn get_name(&self) -> String {
        "Pickaxe".to_string()
    }
}

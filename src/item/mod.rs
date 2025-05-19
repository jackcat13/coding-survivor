use std::any::Any;

use raylib::ffi::Vector2;

pub trait Item {
    fn get_name(&self) -> String;
    fn to_inventory_item(&self) -> InventoryItem;
}

pub struct MapItem {
    pub position: Vector2,
    pub item: Box<dyn Item>,
}

pub struct InventoryItem {
    pub number: i32,
    pub item: Box<dyn Item>,
}

impl PartialEq for InventoryItem {
    fn eq(&self, other: &Self) -> bool {
        self.type_id() == other.type_id()
    }
}

unsafe impl Send for MapItem {}
unsafe impl Send for InventoryItem {}

#[derive(Clone)]
pub struct Pickaxe {
    
}

impl Item for Pickaxe {
    fn get_name(&self) -> String {
        "Pickaxe".to_string()
    }

    fn to_inventory_item(&self) -> InventoryItem {
        InventoryItem { number: 0, item: Box::new(self.clone()) }
    }
}

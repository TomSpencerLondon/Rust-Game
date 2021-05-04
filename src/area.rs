use crate::inventory::{Inventory, Item, ItemType};
use crate::game::GameMessage;

pub struct Area {
    description: String,
    inventory: Inventory
}

impl Area {
    pub fn meadows() -> Area {
        Area {
            description: String::from("Your feet rest upon green meadows."),
            inventory: Inventory::from(vec![
                Item { name: String::from("Potion"), item_type: ItemType::Potion },
                Item { name: String::from("Poison"), item_type: ItemType::Poison } ])
        }
    }

    pub fn look(&self) -> GameMessage {
        GameMessage::new(format!(
            "{}\nYou look around, and see:\n{}",
            self.description, self.inventory
        ))

    }

    pub fn without_item(self, index: usize) -> Area {
        Area { inventory: self.inventory.without(index), ..self }
    }

    pub fn get_from_inventory(&self, index: &usize) -> &Item {
        &self.inventory[index]
    }
}

pub mod consumable;
pub mod equippable;
pub mod error;
pub mod inventory;
pub mod item;

pub mod prelude {
    pub use crate::inventory::Inventory;
    pub use crate::inventory::ItemStack;
    pub use crate::item::Item;
}

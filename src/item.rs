use std::fmt::{Debug, Display};

use crate::consumable::Consumable;

///
/// Item represents an individual Item in an inventory.
/// This includes items such as potions, building materials, and food.
///
/// Only one of each item can exist--i.e., no two items share the
/// same numeric id.
///
pub trait Item: Display + Debug {
    ///
    /// Retrieve name
    ///
    fn get_name(&self) -> &str;

    ///
    /// Update name.
    ///
    /// # Arguments
    ///
    /// * `nme` - replacement name
    ///
    fn set_name(&mut self, nme: &str);

    fn is_stackable(&self) -> bool;
}

use std::fmt::{Debug, Display};

use crate::item::Item;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Consumable {
    name: String,
    effect: String,
}

impl Default for Consumable {
    fn default() -> Self {
        Self {
            name: String::from("[Placeholder]"),
            effect: String::from("[Placeholder]"),
        }
    }
}

impl Item for Consumable {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, nme: &str) {
        self.name = String::from(nme);
    }

    fn is_stackable(&self) -> bool {
        true
    }
}

impl Consumable {
    pub fn new(name: &str, effect: &str) -> Self {
        Self {
            name: name.to_owned(),
            effect: effect.to_owned(),
        }
    }

    pub fn get_effect(&self) -> &str {
        &self.effect
    }

    pub fn set_effect(&mut self, eft: &str) {
        self.effect = String::from(eft);
    }
}

impl std::hash::Hash for Consumable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.effect.hash(state);
    }
}

impl std::fmt::Display for Consumable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "  {}: {}", "Nme", self.get_name())?;
        write!(f, "  {}: {}", "Eft", self.get_effect())?;

        Ok(())
    }
}

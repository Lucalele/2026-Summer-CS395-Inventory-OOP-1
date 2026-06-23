use std::fmt::{Debug, Display};

use crate::item::Item;

#[derive(Clone, Debug)]
pub struct Tool {
    name: String,
    durability: u8,
    speed: u8,
    material: String,
    modifier: String,
    modifier_level: u8,
}

impl Tool {
    pub fn new(
        name: &str,
        durability: u8,
        speed: u8,
        material: &str,
        modifier: &str,
        modifier_level: u8,
    ) -> Self {
        Self {
            name: name.to_owned(),
            durability,
            speed,
            material: material.to_owned(),
            modifier: modifier.to_owned(),
            modifier_level,
        }
    }

    pub fn get_durability(&self) -> u8 {
        self.durability
    }

    pub fn set_durability(&mut self, val: u8) {
        self.durability = val
    }

    pub fn get_speed(&self) -> u8 {
        self.speed
    }

    pub fn set_speed(&mut self, val: u8) {
        self.speed = val;
    }

    pub fn get_material(&self) -> &str {
        &self.material
    }

    pub fn set_material(&mut self, val: &str) {
        self.material = String::from(val);
    }

    pub fn get_modifier(&self) -> (&str, u8) {
        (&self.modifier, self.modifier_level)
    }

    pub fn set_modifier(&mut self, mdr: &str, lvl: u8) {
        self.modifier = String::from(mdr);
        self.modifier_level = lvl;
    }
}

impl std::fmt::Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (mdr, level) = self.get_modifier();
        write!(f, "  {}: {} (Lvl {})", "Mdr", mdr, level)?;

        Ok(())
    }
}

//------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct Armour {
    name: String,
    durability: u8,
    defense: u8,
    material: String,
    modifier: String,
    modifier_level: u8,
    element: String,
}

impl Armour {
    pub fn new(
        name: &str,
        durability: u8,
        defense: u8,
        material: &str,
        modifier: &str,
        modifier_level: u8,
        element: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            durability,
            defense,
            material: material.to_owned(),
            modifier: modifier.to_owned(),
            modifier_level,
            element: element.to_owned(),
        }
    }

    pub fn get_durability(&self) -> u8 {
        self.durability
    }

    pub fn set_durability(&mut self, val: u8) {
        self.durability = val
    }

    pub fn get_defense(&self) -> u8 {
        self.defense
    }

    pub fn set_defense(&mut self, val: u8) {
        self.defense = val;
    }

    pub fn get_material(&self) -> &str {
        &self.material
    }

    pub fn set_material(&mut self, val: &str) {
        self.material = String::from(val);
    }

    pub fn get_modifier(&self) -> (&str, u8) {
        (&self.modifier, self.modifier_level)
    }

    pub fn set_modifier(&mut self, mdr: &str, lvl: u8) {
        self.modifier = String::from(mdr);
        self.modifier_level = lvl;
    }

    pub fn get_element(&self) -> &str {
        &self.element
    }

    pub fn set_element(&mut self, val: &str) {
        self.element = String::from(val);
    }
}

impl std::fmt::Display for Armour {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}

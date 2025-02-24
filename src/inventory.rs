use std::collections::LinkedList;

use crate::consumable::Consumable;
use crate::item::Item;

///
/// A Homogeneous--i.e., uniform--stack of Items.
///
#[derive(Debug)]
pub struct ItemStack {
    ///
    /// The specific type of item out of which this stack is built.
    ///
    item: Box<dyn Item>,

    ///
    /// Represents the number of items in this stack.
    ///
    quantity: usize,
}

impl<'a> Default for ItemStack {
    ///
    /// Create an empty stack composed of Air.
    ///
    fn default() -> Self {
        Self {
            item: Box::new(Consumable::default()),
            quantity: 0,
        }
    }
}

impl ItemStack {
    ///
    /// Create a stack of the desired type.
    ///
    /// # Arguments
    ///
    /// * `base` - Item out of which the stack is composed
    ///
    /// * `qty` - number of items to place in the stack
    ///
    pub fn new<'a>(base: Box<dyn Item>, qty: usize) -> Self {
        Self {
            item: base,
            quantity: qty,
        }
    }

    ///
    /// Retrieve the Item out of which the stack is composed.
    ///
    /// # Returns
    ///
    /// the item that serves as the base
    ///
    pub fn get_item(&self) -> &Box<dyn Item> {
        &self.item
    }

    ///
    /// Retrieve the size of the stack.
    ///
    /// # Returns
    ///
    /// the current number of items
    ///
    pub fn size(&self) -> usize {
        self.quantity
    }

    ///
    /// Increase the size of the stack.
    ///
    /// # Arguments
    ///
    /// * `qty` - number of items to add
    ///
    pub fn add_items(&mut self, qty: usize) {
        self.quantity += qty;
    }

    ///
    /// Does the Item contained in this stack permit stacking?
    ///
    /// This can be less formally phrased, is this a stackable ItemStack?
    ///
    /// # Returns
    ///
    /// true if the addition of items is permitted
    ///
    pub fn permits_stacking(&self) -> bool {
        self.item.is_stackable()
    }
}

impl std::fmt::Display for ItemStack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &*self.item)?;

        if self.permits_stacking() {
            writeln!(f)?;
            write!(f, "  Qty: {}", self.quantity)?;
        }

        Ok(())
    }
}

impl PartialEq for ItemStack {
    fn eq(&self, rhs: &Self) -> bool {
        self.get_item().get_name() == rhs.get_item().get_name()
            && self.permits_stacking() == rhs.permits_stacking()
            && self.size() == rhs.size()
    }
}

///
/// An Inventory is composed of n slots. Each slot may store only
/// one type of item-- specified by *slots*.
///
/// Once all slots are filled, no additional Item types may be
/// stored. Individual slots may contain any number of the same
/// Item-- if the Item is stackable.
///
#[derive(Debug)]
pub struct Inventory {
    ///
    /// Individual item slots-- each ItemStack occupies one slot.
    ///
    slots: LinkedList<ItemStack>,

    ///
    /// Total number of distinct Item types that can be stored.
    ///
    capacity: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SIZE)
    }
}

impl Inventory {
    ///
    /// This is the Default Inventory size.
    ///
    const DEFAULT_SIZE: usize = 10;

    ///
    /// This is utility function that takes two ItemStacks and adds the
    /// number of items in the right- hand side stack to the left-hand side stack.
    ///
    /// # Arguments
    ///
    /// * `lhs` - stack whose size will be increased
    ///
    /// * `rhs` - stack whose size we need to examine
    ///
    pub fn merge_stacks(lhs: &mut ItemStack, rhs: ItemStack) {
        // lhs needs to have items added to it.
        // rhs's size is needed
        // lhs.????(rhs.????)
        lhs.add_items(rhs.size());
    }

    ///
    /// Create an inventory with n slots.
    ///
    /// # Arguments
    ///
    /// * `desiredCapacity` - size of the new Inventory
    ///
    pub fn new(desired_capacity: usize) -> Self {
        Self {
            slots: LinkedList::new(),
            capacity: desired_capacity,
        }
    }

    ///
    /// Determine the number of slots currently in use.
    ///
    pub fn utilized_slots(&self) -> usize {
        return self.slots.len();
    }

    ///
    /// Determine the number of empty (unused) slots.
    ///
    pub fn empty_slots(&self) -> usize {
        self.total_slots() - self.utilized_slots()
    }

    ///
    /// Retrieve the capacity (number of distinct types of items) that self
    /// inventory can store.
    ///
    pub fn total_slots(&self) -> usize {
        self.capacity
    }

    ///
    /// Determine if the inventory is considered full.
    ///
    /// # Returns
    ///
    /// true if the current size is equal to capacity
    ///
    pub fn is_full(&self) -> bool {
        self.empty_slots() == 0
    }

    ///
    /// Determine if the inventory is empty.
    ///
    /// # Returns
    ///
    /// true if current size is zero
    ///
    pub fn is_empty(&self) -> bool {
        self.slots.len() == 0
    }

    ///
    /// Search through all slots (Nodes in the LinkedList) and look for a
    /// matching ItemStack.
    ///
    /// # Arguments
    ///
    /// * `key` - stack for which the search is being conducted
    ///
    /// # Returns
    ///
    /// matching stack if one was found and `null` otherwise
    ///
    pub fn find_matching_item_stack(&mut self, key: &ItemStack) -> Option<&mut ItemStack> {
        self.slots.iter_mut().find(|stack| {
            let lhs_item = stack.get_item();
            let rhs_item = key.get_item();

            lhs_item.get_name() == rhs_item.get_name()
                && lhs_item.is_stackable() == rhs_item.is_stackable()
        })
    }

    ///
    /// This is the standard Linked List append operation from Review 01
    ///
    /// # Arguments
    ///
    /// * `to_add` - data that we want to store in a Node and add to the list
    ///
    pub fn add_item_stack_no_check(&mut self, to_add: ItemStack) {
        self.slots.push_back(to_add);
    }

    ///
    /// Add one or more items to the inventory list.
    ///
    /// # Arguments
    ///
    /// * `stack` - new stack of items to add
    ///
    /// # Returns
    ///
    /// true if *stack* was added and false otherwise
    ///
    pub fn add_items(&mut self, stack: ItemStack) -> bool {
        if let Some(ref mut the_match) = self.find_matching_item_stack(&stack) {
            // If the Item is stackable, add it to the ItemStack
            if the_match.permits_stacking() {
                the_match.add_items(stack.size());

                return true;
            }
        }

        if self.utilized_slots() < self.capacity {
            self.add_item_stack_no_check(stack);
            return true;
        }

        return false;
    }
}

impl std::fmt::Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            " -Used {} of {} slots",
            self.utilized_slots(),
            self.capacity
        )?;

        write!(
            f,
            "{}",
            self.slots
                .iter()
                .map(ItemStack::to_string)
                .collect::<Vec<_>>()
                .join("\n\n")
        )?;

        Ok(())
    }
}

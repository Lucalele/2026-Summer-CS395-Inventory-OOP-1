use hamcrest2::prelude::*;
use rstest::{fixture, rstest};

use rust_inventory::consumable::Consumable;
use rust_inventory::equippable::Tool;
use rust_inventory::prelude::*;

#[fixture]
fn tomato() -> Consumable {
    Consumable::new("Tomato", "Hunger Reduction")
}

#[fixture]
fn shovel() -> Tool {
    Tool::new("Shovel", 20, 3, "Gold", "Unbreaking", 2)
}

#[rstest]
fn test_default() {
    let generic = ItemStack::default();

    let default_item = Consumable::default();
    assert_that!(
        generic.get_item().get_name(),
        is(equal_to(default_item.get_name()))
    );

    assert_that!(generic.size(), equal_to(0));
}

#[rstest]
fn test_constructor(tomato: Consumable) {
    let a_stack = ItemStack::new(Box::new(tomato.clone()), 1);

    assert_that!(a_stack.get_item().get_name(), equal_to(tomato.get_name()));
    assert_that!(
        a_stack.get_item().is_stackable(),
        equal_to(tomato.is_stackable())
    );

    assert_that!(a_stack.size(), equal_to(1));
    assert_that!(a_stack.permits_stacking(), is(true));
}

#[rstest]
pub fn test_add_items_stackable(tomato: Consumable) {
    let mut original_stack = ItemStack::new(Box::new(tomato.clone()), 1);
    original_stack.add_items(11);

    assert_that!(
        original_stack.get_item().get_name(),
        equal_to(tomato.get_name())
    );
    assert_that!(
        original_stack.get_item().is_stackable(),
        equal_to(tomato.is_stackable())
    );
    assert_that!(original_stack.size(), equal_to(12));
    assert_that!(original_stack.permits_stacking(), is(true));

    let another_stack = ItemStack::new(Box::new(tomato.clone()), 1);
    assert_that!(&original_stack, is(not(equal_to(&another_stack))));
    // assert_that!(original_stack.hashCode(), equal_to(another_stack.hashCode()));

    let another_stack = ItemStack::new(Box::new(tomato), 12);
    assert_that!(original_stack, is(equal_to(another_stack)));
}

#[rstest]
pub fn test_display(shovel: Tool, tomato: Consumable) {
    let a_stack = ItemStack::new(Box::new(shovel.clone()), 1);

    assert_that!(a_stack.to_string().find(&shovel.to_string()), is(some()));

    assert_that!(
        a_stack.to_string().find(&format!("  Qty: {}", 1)),
        is(none())
    );

    // Tomatoes are delicious
    let a_stack = ItemStack::new(Box::new(tomato.clone()), 337);

    assert_that!(a_stack.to_string().find(&tomato.to_string()), is(some()));
    assert_that!(
        a_stack.to_string().find(&format!("  Qty: {}", 337)),
        is(some())
    );
}

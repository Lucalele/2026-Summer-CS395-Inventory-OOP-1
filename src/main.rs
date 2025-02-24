use eyre::WrapErr;

use rust_inventory::consumable::Consumable;
use rust_inventory::equippable::{Armour, Tool};
use rust_inventory::prelude::*;

fn main() -> eyre::Result<()> {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() < 2 {
        eyre::bail!("Usage: {} inventory_size", argv[0]);
    }

    let inv_size: usize = argv[1]
        .parse()
        .wrap_err_with(|| format!("'{}' is not a valid number", argv[1]))?;

    let items = create_items();
    let a_bag = Inventory::new(inv_size);
    let (a_bag, results) = process_items(a_bag, items);

    println!("Processing Log:");
    for result in results {
        println!("{}", result);
    }
    println!();

    println!("Storage Summary:");
    println!("{}", a_bag);

    Ok(())
}

fn create_items() -> Vec<Box<dyn Item>> {
    vec![
        Box::new(Tool::new("Pickaxe", 100, 1, "Diamond", "Fortune", 50)),
        Box::new(Tool::new("Shovel", 20, 3, "Gold", "Unbreaking", 2)),
        Box::new(Tool::new("Pickaxe", 100, 1, "Diamond", "Fortune", 5)),
        Box::new(Consumable::new("Speed-II-Potion", "Spd*2")),
        Box::new(Consumable::new("Tomato", "Hunger-10")),
        Box::new(Consumable::new("PotatoCamera", "ImageQuality-97%")),
        Box::new(Consumable::new("PotatoCamera", "ImageQuality-97%")),
        Box::new(Tool::new("Axe", 10, 2, "Stone", "Unbreaking", 2)),
        Box::new(Armour::new(
            "Boots",
            100,
            10,
            "Diamond",
            "Protection",
            3,
            "lightning",
        )),
        Box::new(Armour::new(
            "Boots",
            100,
            10,
            "Diamond",
            "FeatherFalling",
            4,
            "lightning",
        )),
    ]
}

fn process_items(mut a_bag: Inventory, items: Vec<Box<dyn Item>>) -> (Inventory, Vec<String>) {
    let results: Vec<String> = items
        .into_iter()
        .map(|boxed_item| {
            let item_name = String::from(boxed_item.get_name());
            let result = a_bag.add_items(ItemStack::new(boxed_item, 1));

            (item_name, result)
        })
        .map(|(item_name, result)| format!(" ({}) {}", if result { "S" } else { "D" }, item_name))
        .collect();

    (a_bag, results)
}

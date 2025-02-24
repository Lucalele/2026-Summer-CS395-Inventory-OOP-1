use std::hash::{DefaultHasher, Hash, Hasher};

use hamcrest2::prelude::*;
use rstest::{fixture, rstest};

use rust_inventory::consumable::Consumable;
use rust_inventory::prelude::*;

#[fixture]
fn tea() -> Consumable {
    Consumable::new("Green Tea", "Wake Up")
}

#[rstest]
fn test_default(tea: Consumable) {
    let imagination = Consumable::default();

    let generic_ref: &dyn Item = &imagination;

    assert!(imagination.is_stackable());
    assert!(generic_ref.is_stackable());

    assert_that!(imagination.get_name(), is(equal_to("[Placeholder]")));
    assert_that!(imagination.get_effect(), is(equal_to("[Placeholder]")));
}

#[rstest]
fn test_to_string(tea: Consumable) {
    let expected = ["  Nme: Green Tea", "  Eft: Wake Up"].join("\n");

    assert_that!(tea.to_string(), equal_to(expected));
}

#[rstest]
fn test_equals(tea: Consumable) {
    let generic = Consumable::default();
    let more_tea = Consumable::new("Green Tea", "Wake Up");

    assert_that!(&tea, not(equal_to(&generic)));
    assert_that!(&tea, is(equal_to(&more_tea)));

    let more_tea = Consumable::new("Green Tea", "Relax");
    assert_that!(&tea, is(not(equal_to(&more_tea))));
}

#[rstest]
fn test_hash(tea: Consumable) {
    let more_tea = Consumable::new("Green Tea", "Wake Up");
    assert_that!(hash(&tea), equal_to(hash(&more_tea)));

    let more_tea = Consumable::new("Earl Grey", "Wake Up");
    assert_that!(hash(&tea), not(equal_to(hash(&more_tea))));

    let more_tea = Consumable::new("Green Tea", "Relax");
    assert_that!(hash(&tea), not(equal_to(hash(&more_tea))));
}

///
/// Retrieved from std::hash rustdoc, <https://doc.rust-lang.org/std/hash/index.html>
///
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

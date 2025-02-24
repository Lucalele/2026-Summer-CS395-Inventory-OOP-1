use std::hash::{DefaultHasher, Hash, Hasher};

use hamcrest2::prelude::*;
use rstest::{fixture, rstest};

use rust_inventory::equippable::Armour;
use rust_inventory::prelude::*;

#[fixture]
fn fancy_armour() -> Armour {
    Armour::new(
        "Fancy",
        255,
        62,
        "Vibranium",
        "ProcrastinationReduction",
        255,
        "H20",
    )
}

#[rstest]
fn test_default() {
    let generic_armour = Armour::default();
    let generic_ref: &dyn Item = &generic_armour;

    assert_that!(generic_armour.is_stackable(), is(false));
    assert_that!(generic_ref.is_stackable(), is(false));

    // I should really complete this unit test with calls to each of the
    // accessors. However, I will forgo the remaining checks for this test
}

#[rstest]
fn test_to_string(fancy_armour: Armour) {
    let expected = [
        "  Nme: Fancy",
        "  Dur: 255",
        "  Def: 62",
        "  Mtl: Vibranium",
        "  Mdr: ProcrastinationReduction (Lvl 255)",
        "  Emt: H20",
    ]
    .join("\n");

    assert_that!(fancy_armour.to_string(), equal_to(expected));
}

#[rstest]
fn test_equals_and_hash(fancy_armour: Armour) {
    let generic = Armour::default();

    assert_that!(&fancy_armour, not(equal_to(&generic)));
    assert_that!(hash(&fancy_armour), not(equal_to(hash(&generic))));

    let imitation = Armour::new(
        "Fancy",
        255,
        62,
        "Vibranium",
        "ProcrastinationReduction",
        255,
        "H20",
    );
    assert_that!(&fancy_armour, is(equal_to(&imitation)));
    assert_that!(hash(&fancy_armour), equal_to(hash(&imitation)));

    let imitation = Armour::new(
        "Less Fancy",
        255,
        62,
        "Vibranium",
        "ProcrastinationReduction",
        255,
        "H20",
    );
    assert_that!(&fancy_armour, is(not(equal_to(&imitation))));
    assert_that!(hash(&fancy_armour), not(equal_to(hash(&imitation))));

    let imitation = Armour::new(
        "Fancy",
        2,
        62,
        "Vibranium",
        "ProcrastinationReduction",
        255,
        "H20",
    );
    assert_that!(&fancy_armour, is(not(equal_to(&imitation))));
    assert_that!(hash(&fancy_armour), not(equal_to(hash(&imitation))));

    let imitation = Armour::new(
        "Fancy",
        255,
        6,
        "Vibranium",
        "ProcrastinationReduction",
        8,
        "H20",
    );
    assert_that!(&fancy_armour, is(not(equal_to(&imitation))));
    assert_that!(hash(&fancy_armour), not(equal_to(hash(&imitation))));

    let imitation = Armour::new(
        "More Fancy!",
        255,
        62,
        "Nacho Cheese Doritos",
        "ProcrastinationReduction",
        255,
        "H20",
    );
    assert_that!(&fancy_armour, is(not(equal_to(&imitation))));
    assert_that!(hash(&fancy_armour), not(equal_to(hash(&imitation))));

    let imitation = Armour::new(
        "Fancy",
        255,
        62,
        "Vibranium",
        "Java Cloneable Interface",
        255,
        "H20",
    );
    assert_that!(&fancy_armour, is(not(equal_to(&imitation))));
    assert_that!(hash(&fancy_armour), not(equal_to(hash(&imitation))));

    let imitation = Armour::new(
        "Fancy",
        255,
        62,
        "Vibranium",
        "ProcrastinationReduction",
        2,
        "H20",
    );
    assert_that!(&fancy_armour, is(not(equal_to(&imitation))));
    assert_that!(hash(&fancy_armour), not(equal_to(hash(&imitation))));

    let imitation = Armour::new(
        "Fancy",
        255,
        62,
        "Vibranium",
        "ProcrastinationReduction",
        255,
        "Null",
    );
    assert_that!(&fancy_armour, is(not(equal_to(&imitation))));
    assert_that!(hash(&fancy_armour), not(equal_to(hash(&imitation))));
}

///
/// Retrieved from std::hash rustdoc, <https://doc.rust-lang.org/std/hash/index.html>
///
/// TODO: Refactor to avoid duplication between test files.
/// Refer to <https://doc.rust-lang.org/book/ch11-03-test-organization.html>
///
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

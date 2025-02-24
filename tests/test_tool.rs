use std::hash::{DefaultHasher, Hash, Hasher};

use hamcrest2::prelude::*;
use rstest::{fixture, rstest};

use rust_inventory::equippable::Tool;
use rust_inventory::prelude::*;

#[fixture]
fn left_handed_hammer() -> Tool {
    Tool::new(
        "Left-Handed Hammer",
        255,
        62,
        "Titanium",
        "WorkAcceleration",
        255,
    )
}

#[rstest]
fn test_default() {
    let generic_tool = Tool::default();
    let generic_ref: &dyn Item = &generic_tool;

    assert_that!(generic_tool.is_stackable(), is(false));
    assert_that!(generic_ref.is_stackable(), is(false));

    // I should really complete this unit test with calls to each of the
    // accessors. However, I will forgo the remaining checks for this test
}

#[rstest]
fn test_to_string(left_handed_hammer: Tool) {
    let expected = [
        "  Nme: Left-Handed Hammer",
        "  Dur: 255",
        "  Spd: 62",
        "  Mtl: Titanium",
        "  Mdr: WorkAcceleration (Lvl 255)",
    ]
    .join("\n");

    assert_that!(left_handed_hammer.to_string(), equal_to(expected));
}

#[rstest]
fn test_equals_and_hash(left_handed_hammer: Tool) {
    let generic = Tool::default();

    assert_that!(&left_handed_hammer, not(equal_to(&generic)));
    assert_that!(hash(&left_handed_hammer), not(equal_to(hash(&generic))));

    let imitation = Tool::new(
        "Left-Handed Hammer",
        255,
        62,
        "Titanium",
        "WorkAcceleration",
        255,
    );

    assert_that!(&left_handed_hammer, is(equal_to(&imitation)));
    assert_that!(hash(&left_handed_hammer), equal_to(hash(&imitation)));

    let imitation = Tool::new(
        "More Left-Handed Hammer!",
        255,
        62,
        "Titanium",
        "WorkAcceleration",
        255,
    );
    assert_that!(&left_handed_hammer, is(not(equal_to(&imitation))));
    assert_that!(hash(&left_handed_hammer), not(equal_to(hash(&imitation))));

    let imitation = Tool::new(
        "More Left-Handed Hammer!",
        2,
        62,
        "Titanium",
        "WorkAcceleration",
        255,
    );
    assert_that!(&left_handed_hammer, is(not(equal_to(&imitation))));

    assert_that!(hash(&left_handed_hammer), not(equal_to(hash(&imitation))));
    let imitation = Tool::new(
        "Left-Handed Hammer",
        255,
        7,
        "Titanium",
        "WorkAcceleration",
        255,
    );
    assert_that!(&left_handed_hammer, is(not(equal_to(&imitation))));
    assert_that!(hash(&left_handed_hammer), not(equal_to(hash(&imitation))));

    let imitation = Tool::new(
        "Left-Handed Hammer",
        255,
        62,
        "Potato",
        "WorkAcceleration",
        255,
    );
    assert_that!(&left_handed_hammer, is(not(equal_to(&imitation))));
    assert_that!(hash(&left_handed_hammer), not(equal_to(hash(&imitation))));

    let imitation = Tool::new("Left-Handed Hammer", 255, 62, "Titanium", "Hydration", 255);
    assert_that!(&left_handed_hammer, is(not(equal_to(&imitation))));
    assert_that!(hash(&left_handed_hammer), not(equal_to(hash(&imitation))));

    let imitation = Tool::new("Left-Handed Hammer", 255, 62, "Titanium", "Hydration", 2);
    assert_that!(&left_handed_hammer, is(not(equal_to(&imitation))));
    assert_that!(hash(&left_handed_hammer), not(equal_to(hash(&imitation))));
}

///
/// Retrieved from std::hash rustdoc, <https://doc.rust-lang.org/std/hash/index.html>
///
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

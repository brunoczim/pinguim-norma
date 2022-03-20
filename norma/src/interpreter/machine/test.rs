use super::Machine;
use num_bigint::BigUint;
use num_traits::identities::{One, Zero};
use std::cmp::Ordering;

const A_INDEX: usize = 2;
const B_INDEX: usize = 3;

fn make_machine() -> Machine {
    let mut machine = Machine::new(BigUint::from(4u8));
    assert_eq!(machine.create_register("A", BigUint::zero()), A_INDEX);
    assert_eq!(machine.create_register("B", BigUint::from(13u8)), B_INDEX);
    machine
}

#[test]
fn insert() {
    let mut machine = Machine::new(BigUint::from(4u8));
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(4u8));

    let a_index = machine.create_register("A", BigUint::zero());
    assert_eq!(a_index, 2);
    assert_eq!(machine.value(a_index), BigUint::zero());
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(4u8));

    let b_index = machine.create_register("B", BigUint::from(13u8));
    assert_eq!(machine.value(b_index), BigUint::from(13u8));
    assert_eq!(machine.value(a_index), BigUint::zero());
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(4u8));
}

#[test]
fn registers() {
    let machine = make_machine();
    let collected: Vec<_> = machine.registers().collect();

    assert_eq!(
        collected,
        &[
            ("X", BigUint::from(4u8)),
            ("Y", BigUint::zero()),
            ("A", BigUint::zero()),
            ("B", BigUint::from(13u8)),
        ],
    );
}

#[test]
#[should_panic]
fn invalid_get() {
    let machine = make_machine();
    machine.value(9);
}

#[test]
fn inc() {
    let mut machine = make_machine();
    machine.inc(Machine::X_INDEX);
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(5u8));
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(13u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::zero());

    machine.inc(B_INDEX);
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(5u8));
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(14u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::zero());

    machine.inc(A_INDEX);
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(5u8));
    assert_eq!(machine.value(A_INDEX), BigUint::one());
    assert_eq!(machine.value(B_INDEX), BigUint::from(14u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::zero());
}

#[test]
fn dec() {
    let mut machine = make_machine();
    machine.dec(Machine::X_INDEX);
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(3u8));
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(13u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::zero());

    machine.dec(B_INDEX);
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(3u8));
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(12u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::zero());

    machine.dec(A_INDEX);
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(3u8));
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(12u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::zero());
}

#[test]
fn is_zero() {
    let mut machine = make_machine();
    assert!(!machine.is_zero(Machine::X_INDEX));
    assert!(machine.is_zero(A_INDEX));
    assert!(!machine.is_zero(B_INDEX));
    assert!(machine.is_zero(Machine::Y_INDEX));

    machine.inc(A_INDEX);
    assert!(!machine.is_zero(Machine::X_INDEX));
    assert!(!machine.is_zero(A_INDEX));
    assert!(!machine.is_zero(B_INDEX));
    assert!(machine.is_zero(Machine::Y_INDEX));

    machine.inc(B_INDEX);
    assert!(!machine.is_zero(Machine::X_INDEX));
    assert!(!machine.is_zero(A_INDEX));
    assert!(!machine.is_zero(B_INDEX));
    assert!(machine.is_zero(Machine::Y_INDEX));

    machine.dec(Machine::Y_INDEX);
    assert!(!machine.is_zero(Machine::X_INDEX));
    assert!(!machine.is_zero(A_INDEX));
    assert!(!machine.is_zero(B_INDEX));
    assert!(machine.is_zero(Machine::Y_INDEX));

    for _ in 0..10 {
        machine.dec(Machine::X_INDEX);
    }
    assert!(machine.is_zero(Machine::X_INDEX));
    assert!(!machine.is_zero(A_INDEX));
    assert!(!machine.is_zero(B_INDEX));
    assert!(machine.is_zero(Machine::Y_INDEX));
}

#[test]
fn add_const() {
    let mut machine = make_machine();
    machine.add_const(Machine::X_INDEX, &BigUint::from(1234567890u64));
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(1234567894u128));
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(13u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::zero());

    machine.add_const(Machine::Y_INDEX, &BigUint::from(2u64));
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(1234567894u128));
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(13u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::from(2u8));

    machine.add_const(A_INDEX, &BigUint::from(0u64));
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(1234567894u128));
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(13u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::from(2u8));

    machine.add_const(B_INDEX, &BigUint::from(0u64));
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(1234567894u128));
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(13u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::from(2u8));
}

#[test]
fn sub_const() {
    let mut machine = make_machine();
    machine.sub_const(Machine::X_INDEX, &BigUint::from(2u64));
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::from(2u8));
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(13u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::zero());

    machine.sub_const(Machine::X_INDEX, &BigUint::from(1234567890u64));
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::zero());
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(13u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::zero());

    machine.sub_const(A_INDEX, &BigUint::from(1u64));
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::zero());
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(13u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::zero());

    machine.sub_const(A_INDEX, &BigUint::from(0u64));
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::zero());
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(13u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::zero());

    machine.sub_const(B_INDEX, &BigUint::from(0u64));
    assert_eq!(machine.value(Machine::X_INDEX), BigUint::zero());
    assert_eq!(machine.value(A_INDEX), BigUint::zero());
    assert_eq!(machine.value(B_INDEX), BigUint::from(13u8));
    assert_eq!(machine.value(Machine::Y_INDEX), BigUint::zero());
}

#[test]
fn cmp_const() {
    let mut machine = make_machine();
    assert_eq!(
        machine.cmp_const(Machine::X_INDEX, &BigUint::from(4u64)),
        Ordering::Equal
    );
    assert_eq!(
        machine.cmp_const(A_INDEX, &BigUint::from(0u64)),
        Ordering::Equal
    );
    assert_eq!(
        machine.cmp_const(B_INDEX, &BigUint::from(13u64)),
        Ordering::Equal
    );
    assert_eq!(
        machine.cmp_const(Machine::Y_INDEX, &BigUint::from(0u64)),
        Ordering::Equal
    );

    assert_eq!(
        machine.cmp_const(Machine::X_INDEX, &BigUint::from(0u64)),
        Ordering::Greater
    );
    assert_eq!(
        machine.cmp_const(A_INDEX, &BigUint::from(1u64)),
        Ordering::Less
    );
    assert_eq!(
        machine.cmp_const(B_INDEX, &BigUint::from(12u64)),
        Ordering::Greater
    );
    assert_eq!(
        machine.cmp_const(Machine::Y_INDEX, &BigUint::from(9u64)),
        Ordering::Less
    );
}

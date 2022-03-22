use super::{Machine, RegisterId};
use num_bigint::BigUint;
use num_traits::identities::{One, Zero};
use std::cmp::Ordering;

const A_ID: RegisterId = RegisterId { index: 2 };
const B_ID: RegisterId = RegisterId { index: 3 };

fn make_machine() -> Machine {
    let mut machine = Machine::new(BigUint::from(4u8));
    assert_eq!(machine.create_register("A", BigUint::zero()), A_ID);
    assert_eq!(machine.create_register("B", BigUint::from(13u8)), B_ID);
    machine
}

#[test]
fn insert() {
    let mut machine = Machine::new(BigUint::from(4u8));
    assert_eq!(machine.value(RegisterId::X), BigUint::from(4u8));

    let a_id = machine.create_register("A", BigUint::zero());
    assert_eq!(a_id, A_ID);
    assert_eq!(machine.value(a_id), BigUint::zero());
    assert_eq!(machine.value(RegisterId::X), BigUint::from(4u8));

    let b_id = machine.create_register("B", BigUint::from(13u8));
    assert_eq!(machine.value(b_id), BigUint::from(13u8));
    assert_eq!(machine.value(a_id), BigUint::zero());
    assert_eq!(machine.value(RegisterId::X), BigUint::from(4u8));
}

#[test]
fn registers() {
    let machine = make_machine();
    let collected: Vec<_> = machine.registers().collect();

    assert_eq!(
        collected,
        &[
            ("X", RegisterId::X, BigUint::from(4u8)),
            ("Y", RegisterId::Y, BigUint::zero()),
            ("A", A_ID, BigUint::zero()),
            ("B", B_ID, BigUint::from(13u8)),
        ],
    );
}

#[test]
#[should_panic]
fn invalid_get() {
    let machine = make_machine();
    machine.value(RegisterId { index: 9 });
}

#[test]
fn inc() {
    let mut machine = make_machine();
    machine.inc(RegisterId::X);
    assert_eq!(machine.value(RegisterId::X), BigUint::from(5u8));
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(13u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::zero());

    machine.inc(B_ID);
    assert_eq!(machine.value(RegisterId::X), BigUint::from(5u8));
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(14u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::zero());

    machine.inc(A_ID);
    assert_eq!(machine.value(RegisterId::X), BigUint::from(5u8));
    assert_eq!(machine.value(A_ID), BigUint::one());
    assert_eq!(machine.value(B_ID), BigUint::from(14u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::zero());
}

#[test]
fn dec() {
    let mut machine = make_machine();
    machine.dec(RegisterId::X);
    assert_eq!(machine.value(RegisterId::X), BigUint::from(3u8));
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(13u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::zero());

    machine.dec(B_ID);
    assert_eq!(machine.value(RegisterId::X), BigUint::from(3u8));
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(12u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::zero());

    machine.dec(A_ID);
    assert_eq!(machine.value(RegisterId::X), BigUint::from(3u8));
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(12u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::zero());
}

#[test]
fn is_zero() {
    let mut machine = make_machine();
    assert!(!machine.is_zero(RegisterId::X));
    assert!(machine.is_zero(A_ID));
    assert!(!machine.is_zero(B_ID));
    assert!(machine.is_zero(RegisterId::Y));

    machine.inc(A_ID);
    assert!(!machine.is_zero(RegisterId::X));
    assert!(!machine.is_zero(A_ID));
    assert!(!machine.is_zero(B_ID));
    assert!(machine.is_zero(RegisterId::Y));

    machine.inc(B_ID);
    assert!(!machine.is_zero(RegisterId::X));
    assert!(!machine.is_zero(A_ID));
    assert!(!machine.is_zero(B_ID));
    assert!(machine.is_zero(RegisterId::Y));

    machine.dec(RegisterId::Y);
    assert!(!machine.is_zero(RegisterId::X));
    assert!(!machine.is_zero(A_ID));
    assert!(!machine.is_zero(B_ID));
    assert!(machine.is_zero(RegisterId::Y));

    for _ in 0..10 {
        machine.dec(RegisterId::X);
    }
    assert!(machine.is_zero(RegisterId::X));
    assert!(!machine.is_zero(A_ID));
    assert!(!machine.is_zero(B_ID));
    assert!(machine.is_zero(RegisterId::Y));
}

#[test]
fn add_const() {
    let mut machine = make_machine();
    machine.add_const(RegisterId::X, &BigUint::from(1234567890u64));
    assert_eq!(machine.value(RegisterId::X), BigUint::from(1234567894u128));
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(13u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::zero());

    machine.add_const(RegisterId::Y, &BigUint::from(2u64));
    assert_eq!(machine.value(RegisterId::X), BigUint::from(1234567894u128));
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(13u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::from(2u8));

    machine.add_const(A_ID, &BigUint::from(0u64));
    assert_eq!(machine.value(RegisterId::X), BigUint::from(1234567894u128));
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(13u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::from(2u8));

    machine.add_const(B_ID, &BigUint::from(0u64));
    assert_eq!(machine.value(RegisterId::X), BigUint::from(1234567894u128));
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(13u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::from(2u8));
}

#[test]
fn sub_const() {
    let mut machine = make_machine();
    machine.sub_const(RegisterId::X, &BigUint::from(2u64));
    assert_eq!(machine.value(RegisterId::X), BigUint::from(2u8));
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(13u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::zero());

    machine.sub_const(RegisterId::X, &BigUint::from(1234567890u64));
    assert_eq!(machine.value(RegisterId::X), BigUint::zero());
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(13u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::zero());

    machine.sub_const(A_ID, &BigUint::from(1u64));
    assert_eq!(machine.value(RegisterId::X), BigUint::zero());
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(13u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::zero());

    machine.sub_const(A_ID, &BigUint::from(0u64));
    assert_eq!(machine.value(RegisterId::X), BigUint::zero());
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(13u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::zero());

    machine.sub_const(B_ID, &BigUint::from(0u64));
    assert_eq!(machine.value(RegisterId::X), BigUint::zero());
    assert_eq!(machine.value(A_ID), BigUint::zero());
    assert_eq!(machine.value(B_ID), BigUint::from(13u8));
    assert_eq!(machine.value(RegisterId::Y), BigUint::zero());
}

#[test]
fn cmp_const() {
    let mut machine = make_machine();
    assert_eq!(
        machine.cmp_const(RegisterId::X, &BigUint::from(4u64)),
        Ordering::Equal
    );
    assert_eq!(machine.cmp_const(A_ID, &BigUint::from(0u64)), Ordering::Equal);
    assert_eq!(machine.cmp_const(B_ID, &BigUint::from(13u64)), Ordering::Equal);
    assert_eq!(
        machine.cmp_const(RegisterId::Y, &BigUint::from(0u64)),
        Ordering::Equal
    );

    assert_eq!(
        machine.cmp_const(RegisterId::X, &BigUint::from(0u64)),
        Ordering::Greater
    );
    assert_eq!(machine.cmp_const(A_ID, &BigUint::from(1u64)), Ordering::Less);
    assert_eq!(
        machine.cmp_const(B_ID, &BigUint::from(12u64)),
        Ordering::Greater
    );
    assert_eq!(
        machine.cmp_const(RegisterId::Y, &BigUint::from(9u64)),
        Ordering::Less
    );
}

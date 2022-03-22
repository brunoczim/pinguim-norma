use super::SymbolTable;

const FOO_ID: usize = 0;
const BAR_ID: usize = 1;
const BAZ_ID: usize = 2;

fn make_table() -> SymbolTable<String, usize> {
    let mut table = SymbolTable::empty();
    table.create("foo");
    table.create("bar");
    table.create("baz");
    table
}

#[test]
fn try_create() {
    let mut table = SymbolTable::empty();
    assert_eq!(table.try_create(String::from("foo")), Ok(FOO_ID));
    assert_eq!(table.try_create(String::from("bar")), Ok(BAR_ID));
    assert_eq!(table.try_create(String::from("baz")), Ok(BAZ_ID));
    assert_eq!(table.try_create(String::from("foo")), Err(FOO_ID));
}

#[test]
fn create_success() {
    let mut table = SymbolTable::<String, usize>::empty();
    assert_eq!(table.create("foo"), FOO_ID);
    assert_eq!(table.create("bar"), BAR_ID);
    assert_eq!(table.create("baz"), BAZ_ID);
}

#[test]
#[should_panic]
fn create_failure() {
    let mut table = SymbolTable::<String, usize>::empty();
    assert_eq!(table.create("foo"), FOO_ID);
    assert_eq!(table.create("bar"), BAR_ID);
    table.create("foo");
}

#[test]
fn insert() {
    let mut table = SymbolTable::<String, usize>::empty();
    assert_eq!(table.insert(String::from("foo")), FOO_ID);
    assert_eq!(table.insert(String::from("bar")), BAR_ID);
    assert_eq!(table.insert(String::from("baz")), BAZ_ID);
    assert_eq!(table.insert(String::from("foo")), FOO_ID);
}

#[test]
fn try_to_id() {
    let table = make_table();
    assert_eq!(table.try_symbol_to_id("foo"), Some(FOO_ID));
    assert_eq!(table.try_symbol_to_id("bar"), Some(BAR_ID));
    assert_eq!(table.try_symbol_to_id("baz"), Some(BAZ_ID));
    assert_eq!(table.try_symbol_to_id("abc"), None);
    assert_eq!(table.try_symbol_to_id(""), None);
}

#[test]
fn to_id_success() {
    let table = make_table();
    assert_eq!(table.symbol_to_id("foo"), FOO_ID);
    assert_eq!(table.symbol_to_id("bar"), BAR_ID);
    assert_eq!(table.symbol_to_id("baz"), BAZ_ID);
}

#[test]
#[should_panic]
fn to_id_failure() {
    let table = make_table();
    table.symbol_to_id("abc");
}

#[test]
fn try_to_symbol() {
    let table = make_table();
    assert_eq!(table.try_id_to_symbol(FOO_ID).map(String::as_ref), Some("foo"));
    assert_eq!(table.try_id_to_symbol(BAR_ID).map(String::as_ref), Some("bar"));
    assert_eq!(table.try_id_to_symbol(BAZ_ID).map(String::as_ref), Some("baz"));
    assert_eq!(table.try_id_to_symbol(90), None);
}

#[test]
fn to_symbol_success() {
    let table = make_table();
    assert_eq!(table.id_to_symbol(FOO_ID), "foo");
    assert_eq!(table.id_to_symbol(BAR_ID), "bar");
    assert_eq!(table.id_to_symbol(BAZ_ID), "baz");
}

#[test]
#[should_panic]
fn to_symbol_failure() {
    let table = make_table();
    table.id_to_symbol(90);
}

#[test]
fn contains_symbol() {
    let table = make_table();
    assert!(table.contains_symbol("foo"));
    assert!(table.contains_symbol("bar"));
    assert!(table.contains_symbol("baz"));
    assert!(!table.contains_symbol("aaaaaa"));
    assert!(!table.contains_symbol(""));
}

#[test]
fn contains_id() {
    let table = make_table();
    assert!(table.contains_id(FOO_ID));
    assert!(table.contains_id(BAR_ID));
    assert!(table.contains_id(BAZ_ID));
    assert!(!table.contains_id(90));
}

#[test]
fn iter() {
    let table = make_table();
    let collected: Vec<_> =
        table.iter().map(|(sym, id)| (sym.as_str(), id)).collect();

    assert_eq!(collected, &[("foo", FOO_ID), ("bar", BAR_ID), ("baz", BAZ_ID)]);
}

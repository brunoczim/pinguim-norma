use super::SymbolTable;

const FOO_INDEX: usize = 0;
const BAR_INDEX: usize = 1;
const BAZ_INDEX: usize = 2;

fn make_table() -> SymbolTable {
    let mut table = SymbolTable::empty();
    table.create("foo");
    table.create("bar");
    table.create("baz");
    table
}

#[test]
fn try_create() {
    let mut table = SymbolTable::empty();
    assert_eq!(table.try_create(String::from("foo")), Ok(FOO_INDEX));
    assert_eq!(table.try_create(String::from("bar")), Ok(BAR_INDEX));
    assert_eq!(table.try_create(String::from("baz")), Ok(BAZ_INDEX));
    assert_eq!(table.try_create(String::from("foo")), Err(FOO_INDEX));
}

#[test]
fn create_success() {
    let mut table = SymbolTable::empty();
    assert_eq!(table.create("foo"), FOO_INDEX);
    assert_eq!(table.create("bar"), BAR_INDEX);
    assert_eq!(table.create("baz"), BAZ_INDEX);
}

#[test]
#[should_panic]
fn create_failure() {
    let mut table = SymbolTable::empty();
    assert_eq!(table.create("foo"), FOO_INDEX);
    assert_eq!(table.create("bar"), BAR_INDEX);
    table.create("foo");
}

#[test]
fn insert() {
    let mut table = SymbolTable::empty();
    assert_eq!(table.insert(String::from("foo")), FOO_INDEX);
    assert_eq!(table.insert(String::from("bar")), BAR_INDEX);
    assert_eq!(table.insert(String::from("baz")), BAZ_INDEX);
    assert_eq!(table.insert(String::from("foo")), FOO_INDEX);
}

#[test]
fn try_to_index() {
    let table = make_table();
    assert_eq!(table.try_symbol_to_index("foo"), Some(FOO_INDEX));
    assert_eq!(table.try_symbol_to_index("bar"), Some(BAR_INDEX));
    assert_eq!(table.try_symbol_to_index("baz"), Some(BAZ_INDEX));
    assert_eq!(table.try_symbol_to_index("abc"), None);
    assert_eq!(table.try_symbol_to_index(""), None);
}

#[test]
fn to_index_success() {
    let table = make_table();
    assert_eq!(table.symbol_to_index("foo"), FOO_INDEX);
    assert_eq!(table.symbol_to_index("bar"), BAR_INDEX);
    assert_eq!(table.symbol_to_index("baz"), BAZ_INDEX);
}

#[test]
#[should_panic]
fn to_index_failure() {
    let table = make_table();
    table.symbol_to_index("abc");
}

#[test]
fn try_to_symbol() {
    let table = make_table();
    assert_eq!(table.try_index_to_symbol(FOO_INDEX), Some("foo"));
    assert_eq!(table.try_index_to_symbol(BAR_INDEX), Some("bar"));
    assert_eq!(table.try_index_to_symbol(BAZ_INDEX), Some("baz"));
    assert_eq!(table.try_index_to_symbol(90), None);
}

#[test]
fn to_symbol_success() {
    let table = make_table();
    assert_eq!(table.index_to_symbol(FOO_INDEX), "foo");
    assert_eq!(table.index_to_symbol(BAR_INDEX), "bar");
    assert_eq!(table.index_to_symbol(BAZ_INDEX), "baz");
}

#[test]
#[should_panic]
fn to_symbol_failure() {
    let table = make_table();
    table.index_to_symbol(90);
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
fn contains_index() {
    let table = make_table();
    assert!(table.contains_index(FOO_INDEX));
    assert!(table.contains_index(BAR_INDEX));
    assert!(table.contains_index(BAZ_INDEX));
    assert!(!table.contains_index(90));
}

#[test]
fn iter() {
    let table = make_table();
    let collected: Vec<_> = table.iter().collect();

    assert_eq!(collected, &["foo", "bar", "baz"]);
}

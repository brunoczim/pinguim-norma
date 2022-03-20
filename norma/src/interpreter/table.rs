use indexmap::{set, IndexSet};

#[inline(never)]
#[cold]
fn symbol_already_exist(symbol: &str, index: usize) -> ! {
    panic!("Symbol {} already exist at index {}", symbol, index)
}

#[inline(never)]
#[cold]
fn symbol_does_not_exist(symbol: &str) -> ! {
    panic!("Symbol {} does not exist", symbol)
}

#[inline(never)]
#[cold]
fn index_out_of_bounds(index: usize, len: usize) -> ! {
    panic!("Symbol index {} is out of bounds (length {})", index, len)
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SymbolTable {
    symbols: IndexSet<String>,
}

impl SymbolTable {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn try_create(&mut self, symbol: String) -> Result<usize, usize> {
        let (index, created) = self.symbols.insert_full(symbol);
        if created {
            Ok(index)
        } else {
            Err(index)
        }
    }

    pub fn create(&mut self, symbol: &str) -> usize {
        match self.try_create(symbol.to_owned()) {
            Ok(index) => index,
            Err(index) => symbol_already_exist(symbol, index),
        }
    }

    pub fn insert(&mut self, symbol: String) -> usize {
        self.try_create(symbol).unwrap_or_else(|index| index)
    }

    pub fn try_symbol_to_index(&self, symbol: &str) -> Option<usize> {
        self.symbols.get_index_of(symbol)
    }

    pub fn symbol_to_index(&self, symbol: &str) -> usize {
        match self.try_symbol_to_index(symbol) {
            Some(index) => index,
            None => symbol_does_not_exist(symbol),
        }
    }

    pub fn try_index_to_symbol(&self, index: usize) -> Option<&str> {
        self.symbols.get_index(index).map(String::as_ref)
    }

    pub fn index_to_symbol(&self, index: usize) -> &str {
        match self.try_index_to_symbol(index) {
            Some(symbol) => symbol,
            None => index_out_of_bounds(index, self.symbols.len()),
        }
    }

    pub fn contains_symbol(&self, symbol: &str) -> bool {
        self.try_symbol_to_index(symbol).is_some()
    }

    pub fn contains_index(&self, index: usize) -> bool {
        self.try_index_to_symbol(index).is_some()
    }

    pub fn iter(&self) -> Symbols {
        Symbols { iter: self.symbols.iter() }
    }
}

impl<'table> IntoIterator for &'table SymbolTable {
    type Item = &'table str;
    type IntoIter = Symbols<'table>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
pub struct Symbols<'table> {
    iter: set::Iter<'table, String>,
}

impl<'table> Iterator for Symbols<'table> {
    type Item = &'table str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(String::as_ref)
    }
}

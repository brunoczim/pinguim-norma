#[cfg(test)]
mod test;

use indexmap::{set, IndexSet};
use std::{borrow::Borrow, fmt, hash::Hash, marker::PhantomData};

#[inline(never)]
#[cold]
fn symbol_already_exist<K>(symbol: K, index: usize) -> !
where
    K: fmt::Debug,
{
    panic!("Symbol {:?} already exist at index {}", symbol, index)
}

#[inline(never)]
#[cold]
fn symbol_does_not_exist<K>(symbol: K) -> !
where
    K: fmt::Debug,
{
    panic!("Symbol {:?} does not exist", symbol)
}

#[inline(never)]
#[cold]
fn index_out_of_bounds(index: usize, len: usize) -> ! {
    panic!("Symbol index {} is out of bounds (length {})", index, len)
}

pub trait Id: Copy {
    fn from_index(index: usize) -> Self;

    fn into_index(self) -> usize;
}

impl<T> Id for T
where
    T: Into<usize> + Copy,
    usize: Into<T>,
{
    fn from_index(index: usize) -> Self {
        index.into()
    }

    fn into_index(self) -> usize {
        self.into()
    }
}

pub struct SymbolTable<K, I>
where
    K: Hash + Eq,
    I: Id,
{
    symbols: IndexSet<K>,
    _marker: PhantomData<I>,
}

impl<K, I> Clone for SymbolTable<K, I>
where
    K: Hash + Eq + Clone,
    I: Id,
{
    fn clone(&self) -> Self {
        Self { symbols: self.symbols.clone(), _marker: self._marker.clone() }
    }
}

impl<K, I> fmt::Debug for SymbolTable<K, I>
where
    K: Hash + Eq + fmt::Debug,
    I: Id,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_struct("SymbolTable")
            .field("symbols", &self.symbols)
            .finish()
    }
}

impl<K, I> PartialEq for SymbolTable<K, I>
where
    K: Hash + Eq,
    I: Id,
{
    fn eq(&self, other: &Self) -> bool {
        self.symbols.iter().eq(other.symbols.iter())
    }
}

impl<K, I> Eq for SymbolTable<K, I>
where
    K: Hash + Eq,
    I: Id,
{
}

impl<K, I> Default for SymbolTable<K, I>
where
    K: Hash + Eq,
    I: Id,
{
    fn default() -> Self {
        Self { symbols: IndexSet::new(), _marker: PhantomData }
    }
}

impl<K, I> SymbolTable<K, I>
where
    K: Hash + Eq,
    I: Id,
{
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn try_create(&mut self, symbol: K) -> Result<I, I> {
        let (index, created) = self.symbols.insert_full(symbol);
        if created {
            Ok(I::from_index(index))
        } else {
            Err(I::from_index(index))
        }
    }

    pub fn create<Q>(&mut self, symbol: &Q) -> I
    where
        K: Borrow<Q>,
        Q: Hash + Eq + fmt::Debug + ToOwned<Owned = K> + ?Sized,
    {
        match self.try_create(symbol.to_owned()) {
            Ok(id) => id,
            Err(id) => symbol_already_exist(symbol, id.into_index()),
        }
    }

    pub fn insert(&mut self, symbol: K) -> I {
        self.try_create(symbol).unwrap_or_else(|id| id)
    }

    pub fn try_symbol_to_id<Q>(&self, symbol: &Q) -> Option<I>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.symbols.get_index_of(symbol).map(I::from_index)
    }

    pub fn symbol_to_id<Q>(&self, symbol: &Q) -> I
    where
        K: Borrow<Q>,
        Q: Hash + Eq + fmt::Debug + ?Sized,
    {
        match self.try_symbol_to_id(symbol) {
            Some(id) => id,
            None => symbol_does_not_exist(symbol),
        }
    }

    pub fn try_id_to_symbol(&self, id: I) -> Option<&K> {
        self.symbols.get_index(id.into_index())
    }

    pub fn id_to_symbol(&self, id: I) -> &K {
        match self.try_id_to_symbol(id) {
            Some(symbol) => symbol,
            None => index_out_of_bounds(id.into_index(), self.symbols.len()),
        }
    }

    pub fn contains_symbol<Q>(&self, symbol: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.try_symbol_to_id(symbol).is_some()
    }

    pub fn contains_id(&self, id: I) -> bool {
        self.try_id_to_symbol(id).is_some()
    }

    pub fn iter(&self) -> Symbols<K, I> {
        Symbols {
            iter: self.symbols.iter(),
            curr_index: 0,
            _marker: PhantomData,
        }
    }
}

impl<'table, K, I> IntoIterator for &'table SymbolTable<K, I>
where
    K: Hash + Eq,
    I: Id,
{
    type Item = (&'table K, I);
    type IntoIter = Symbols<'table, K, I>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Symbols<'table, K, I>
where
    K: Hash + Eq,
    I: Id,
{
    iter: set::Iter<'table, K>,
    curr_index: usize,
    _marker: PhantomData<I>,
}

impl<'table, K, I> fmt::Debug for Symbols<'table, K, I>
where
    K: Hash + Eq + fmt::Debug,
    I: Id,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_struct("Symbols")
            .field("iter", &self.iter)
            .field("curr_index", &self.curr_index)
            .finish()
    }
}

impl<'table, K, I> Iterator for Symbols<'table, K, I>
where
    K: Hash + Eq,
    I: Id,
{
    type Item = (&'table K, I);

    fn next(&mut self) -> Option<Self::Item> {
        let symbol = self.iter.next()?;
        let index = self.curr_index;
        self.curr_index += 1;
        Some((symbol, I::from_index(index)))
    }
}
